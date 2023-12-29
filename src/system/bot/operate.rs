use crate::{
    component::{Actor, Bot, Inertia},
    model::ActorAction,
    util::{
        ext::{TransformExt, Vec2Ext},
        math::{angle_difference, find_meet_point},
    },
};
use bevy::{
    math::{Vec2, Vec3Swizzles},
    prelude::{Query, Transform, With},
};

const DEBUG_TEAMMATES: bool = false;

pub fn operate(
    mut bots: Query<(&Bot, &mut Actor, &Transform, &Inertia)>,
    actors: Query<(&Transform, &Inertia), With<Actor>>,
) {
    for (bot, mut actor, transform, inertia) in bots.iter_mut() {
        actor.reset_actions();

        let allow_spread_out =
            sub_system::follow_enemy(bot, &mut actor, transform, inertia, &actors);

        if allow_spread_out {
            sub_system::spread_out(bot, &mut actor, transform, &actors);
        }
    }
}

mod sub_system {
    use super::*;
    use crate::util::DEBUG_LINES;
    use bevy::prelude::Color;

    pub fn follow_enemy(
        bot: &Bot,
        actor: &mut Actor,
        transform: &Transform,
        inertia: &Inertia,
        actors: &Query<(&Transform, &Inertia), With<Actor>>,
    ) -> bool {
        let mut allow_spread_out = true;

        if let Some((enemy_position, enemy_velocity)) = bot
            .enemy
            .and_then(|e| actors.get(e).ok())
            .map(|e| (e.0.translation.xy(), e.1.velocity))
        {
            let position = transform.translation.xy();
            actor.actions |= ActorAction::MovementForward;

            // TODO: count enemy body radius instead of self
            if (position - enemy_position)
                .is_shorter_than(actor.config.melee_distance + actor.config.radius)
            {
                // enemy is close, attack
                actor.actions |= ActorAction::Attack;
                actor.look_at = Some(position.angle_to(enemy_position));
                allow_spread_out = false;
            } else {
                // otherwise go to the meet point
                let meet_position =
                    find_meet_point(position, inertia.velocity, enemy_position, enemy_velocity);

                let meet_distance = (position - meet_position).length_squared();

                if is_close(meet_distance, bot.spread) {
                    // meet point is near, no need to spread out
                    allow_spread_out = false;
                }

                if actor.stamina > bot.stamina_min && is_far(meet_distance, bot.sprint_distance) {
                    // enemy is far, sprint
                    actor.actions |= ActorAction::Sprint;
                }

                actor.look_at = Some(position.angle_to(meet_position));
            }
        }

        return allow_spread_out;
    }

    pub fn spread_out(
        bot: &Bot,
        actor: &mut Actor,
        transform: &Transform,
        actors: &Query<(&Transform, &Inertia), With<Actor>>,
    ) {
        let position = transform.translation.xy();
        let mut teammates_position_sum = Vec2::ZERO;
        let mut teammates_position_sum_weight = 0.0;

        for teammate in &bot.teammates {
            let Ok(teammate_position) = actors.get(*teammate).map(|a| a.0.translation.xy()) else {
                continue;
            };

            let teammate_distance = (position - teammate_position).length_squared();

            if teammate_distance < bot.spread * bot.spread {
                let weight = 1.0 - teammate_distance.sqrt() / bot.spread;
                teammates_position_sum += teammate_position * weight;
                teammates_position_sum_weight += weight;
            }
        }

        if teammates_position_sum_weight == 0.0 {
            return;
        }

        let teammates_position = teammates_position_sum / teammates_position_sum_weight;
        let teammates_distance = (position - teammates_position).length_squared();

        if is_close(teammates_distance, bot.spread) {
            let look_at = actor.look_at.unwrap_or_else(|| transform.direction());
            let turn = angle_difference(look_at, teammates_position.angle_to(position));
            let closeness = 1.0 - teammates_distance.sqrt() / bot.spread; // the closer teammates, the faster spread out
            actor.look_at = Some(look_at + turn * closeness * bot.spread_angular_factor);
            actor.actions |= ActorAction::MovementForward;

            if closeness > 0.75 {
                actor.actions -= ActorAction::Sprint;
            }

            if DEBUG_TEAMMATES {
                DEBUG_LINES.ln(
                    position,
                    teammates_position,
                    Color::rgba(0.0, 1.0, 0.0, closeness),
                );
            }
        }
    }
}

fn is_close(distance_squared: f32, limit: f32) -> bool {
    return distance_squared < limit * limit;
}

fn is_far(distance_squared: f32, limit: f32) -> bool {
    return !is_close(distance_squared, limit);
}
