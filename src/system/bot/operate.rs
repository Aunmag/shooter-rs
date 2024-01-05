use crate::{
    component::{Actor, Bot, BotShootingState, Inertia, Weapon},
    model::{ActorAction, Moving},
    util::{
        ext::{TransformExt, Vec2Ext},
        math::angle_difference,
        traits::{WithPosition, WithPositionAndVelocity, WithVelocity},
        GIZMOS,
    },
};
use bevy::{
    ecs::system::Res,
    math::{Vec2, Vec3Swizzles},
    prelude::{Color, Query, Transform, With},
    time::Time,
};
use std::{ops::Div, time::Duration};

const DEBUG_TEAMMATES: bool = false;
const DEBUG_AIM: bool = false;

pub fn operate(
    mut bots: Query<(&mut Bot, &mut Actor, &Transform, &Inertia, Option<&Weapon>)>,
    actors: Query<(&Transform, &Inertia), With<Actor>>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (mut bot, mut actor, transform, inertia, weapon) in bots.iter_mut() {
        actor.reset_actions();

        let enemy = bot.enemy.and_then(|e| actors.get(e).ok()).map(|e| Moving {
            position: e.0.translation.xy(),
            velocity: e.1.velocity,
        });

        if bot.enemy.is_some() && enemy.is_none() {
            // enemy no longer exists. force new enemy search now
            let reaction = bot.config.reaction;
            bot.enemy = None;
            bot.update_timer.set(time + reaction);
        }

        let mut handler = BotHandler {
            bot: &mut bot,
            actor: &mut actor,
            transform,
            velocity: inertia.velocity,
            spread_out: SpreadOut::Full,
        };

        if let Some(enemy) = enemy {
            if let Some(weapon) = weapon {
                handler.attack_enemy_armed(&enemy, weapon, time);
            } else {
                handler.attack_enemy_melee(&enemy);
            }
        } else {
            handler.idle();
        }

        match handler.spread_out {
            SpreadOut::Full => {
                handler.spread_out(true, &actors);
            }
            SpreadOut::Restricted => {
                handler.spread_out(false, &actors);
            }
            SpreadOut::Disallowed => {}
        }
    }
}

struct BotHandler<'a> {
    bot: &'a mut Bot,
    actor: &'a mut Actor,
    transform: &'a Transform,
    velocity: Vec2,
    spread_out: SpreadOut,
}

impl<'a> BotHandler<'a> {
    fn attack_enemy_armed(&mut self, target: &Moving, weapon: &Weapon, time: Duration) {
        if self.is_close(&target.position, self.bot.config.shoot_distance_min) {
            // don't come too close
            self.actor.movement += Vec2::BACK / 1.5;
            self.spread_out = SpreadOut::Disallowed;
        }

        if !weapon.is_armed() {
            self.actor.actions |= ActorAction::Reload;
            return;
        }

        if self.can_aim_at(target.position) {
            if self.spread_out != SpreadOut::Disallowed {
                self.spread_out = SpreadOut::Restricted;
            }

            if self.bot.config.is_silly
                && self.is_far(&target.position, self.bot.config.shoot_distance_min * 1.25)
            {
                // shoot while walking
                self.actor.movement += Vec2::FRONT;
            }

            self.bot.set_shooting_target(true, time);

            let shooting_state = self
                .bot
                .get_shooting_state(weapon.config.is_automatic, time);

            let debug_color;
            let is_aimed = self.is_aimed_at(target.position);

            if shooting_state == BotShootingState::Shoot && (is_aimed || self.bot.was_burst_fire) {
                self.actor.actions |= ActorAction::Attack;
                self.bot.was_burst_fire = weapon.config.is_automatic;
                debug_color = Color::RED;
            } else {
                // keep aim ony while not attacking, otherwise recoil won't work
                // don't turn further if is already aimed, otherwise it would be too precise
                if !is_aimed {
                    self.look_at_position(target.position);
                }

                self.bot.was_burst_fire = false;

                match shooting_state {
                    BotShootingState::Prepare => {
                        debug_color = Color::GREEN;
                    }
                    BotShootingState::Shoot => {
                        debug_color = Color::ORANGE;
                    }
                    BotShootingState::Pause => {
                        debug_color = Color::YELLOW;
                    }
                }
            }

            if DEBUG_AIM {
                GIZMOS.ln(
                    self.position(),
                    self.position()
                        + Vec2::from_length(
                            self.bot.config.shoot_distance_max,
                            self.transform.direction(),
                        ),
                    debug_color,
                );
            }
        } else {
            self.bot.set_shooting_target(false, time);
            self.chase(target);
        }
    }

    fn attack_enemy_melee(&mut self, target: &Moving) {
        // TODO: count enemy body radius instead of self
        let melee_distance = self.actor.config.melee_distance + self.actor.config.radius;

        if self.is_close(&target.position, melee_distance) {
            // enemy is close, attack
            self.look_at_position(target.position);
            self.actor.actions |= ActorAction::Attack;
            self.actor.movement += Vec2::FRONT;
            self.spread_out = SpreadOut::Disallowed;
        } else {
            // otherwise just chase
            self.chase(target);
        }
    }

    fn chase(&mut self, target: &Moving) {
        let meet = self.find_meet(target);

        if self.is_close(&meet, self.bot.config.spread) {
            // meet point is near, no need to spread out
            self.spread_out = SpreadOut::Disallowed;
        }

        if self.can_sprint() && self.is_far(&meet, self.bot.config.sprint_distance) {
            // enemy is far, sprint
            self.actor.actions |= ActorAction::Sprint;
        }

        self.look_at_position(meet);
        self.actor.movement += Vec2::FRONT;
    }

    fn idle(&mut self) {
        self.look_at_direction(self.bot.idle_direction);

        if self.bot.idle_movement {
            self.actor.movement += Vec2::FRONT;
        }
    }

    fn spread_out(&mut self, is_full: bool, actors: &Query<(&Transform, &Inertia), With<Actor>>) {
        let mut teammates_position_sum = Vec2::ZERO;
        let mut teammates_position_sum_weight = 0.0;

        for teammate in &self.bot.teammates {
            let Ok(teammate_position) = actors.get(*teammate).map(|a| a.0.translation.xy()) else {
                continue;
            };

            let teammate_distance = self.distance_squared(&teammate_position);

            if teammate_distance < self.bot.config.spread * self.bot.config.spread {
                let weight = 1.0 - teammate_distance.sqrt() / self.bot.config.spread;
                teammates_position_sum += teammate_position * weight;
                teammates_position_sum_weight += weight;
            }
        }

        if teammates_position_sum_weight == 0.0 {
            return;
        }

        let teammates_position = teammates_position_sum / teammates_position_sum_weight;
        let teammates_distance = self.distance_squared(&teammates_position);

        if DEBUG_TEAMMATES {
            GIZMOS.ln(self.position(), teammates_position, Color::GREEN);
        }

        if teammates_distance < self.bot.config.spread * self.bot.config.spread {
            if is_full {
                let look_at = self.get_look_at(); // TODO: maybe use just direction or movement?
                let turn = angle_difference(look_at, teammates_position.angle_to(self.position())); // turn away from teammate
                let closeness = 1.0 - teammates_distance.sqrt() / self.bot.config.spread; // the closer teammates, the faster spread out
                self.look_at_direction(look_at + turn * closeness * self.bot.config.spread_force);
                self.actor.movement += Vec2::FRONT;

                // cancel splint if group is too tight
                if closeness > 0.75 {
                    self.actor.actions -= ActorAction::Sprint;
                }
            } else {
                // TODO: simplify
                // find subjection direction to teammates, and go in opposite direction
                self.actor.movement -= (teammates_position - self.position())
                    .normalize_or_zero()
                    .rotate_by(-self.transform.direction())
                    .div(2.0);
            }
        }
    }

    fn look_at_direction(&mut self, direction: f32) {
        self.actor.look_at = Some(direction);
    }

    fn look_at_position(&mut self, target: Vec2) {
        self.actor.look_at = Some(self.angle_to(&target));
    }

    fn get_look_at(&self) -> f32 {
        return self
            .actor
            .look_at
            .unwrap_or_else(|| self.transform.direction());
    }

    fn is_aimed_at(&self, target: Vec2) -> bool {
        return angle_difference(self.transform.direction(), self.angle_to(&target)).abs()
            < self.bot.config.angular_deviation;
    }

    fn can_sprint(&self) -> bool {
        return !self.actor.actions.contains(ActorAction::Reload);
    }

    fn can_aim_at(&self, target: Vec2) -> bool {
        return !self.actor.actions.contains(ActorAction::Reload)
            && self.is_close(&target, self.bot.config.shoot_distance_max);
    }
}

impl<'a> WithPosition for BotHandler<'a> {
    fn position(&self) -> Vec2 {
        return self.transform.translation.truncate();
    }
}

impl<'a> WithVelocity for BotHandler<'a> {
    fn velocity(&self) -> Vec2 {
        return self.velocity;
    }
}

#[derive(PartialEq, Eq)]
enum SpreadOut {
    Full,
    Restricted,
    Disallowed,
}
