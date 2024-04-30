use super::component::BotShootingState;
use crate::{
    component::Actor,
    model::ActorAction,
    plugin::{
        bot::{Bot, BotConfig},
        debug::{debug_circle, debug_line},
        kinetics::Kinetics,
        Weapon,
    },
    util::{
        ext::{TransformExt, Vec2Ext},
        math::angle_difference,
        traits::{WithPosition, WithPositionAndVelocity, WithVelocity},
    },
};
use bevy::{
    ecs::system::Res,
    math::{Vec2, Vec3Swizzles},
    prelude::{Color, Query, Transform, With},
    time::Time,
};
use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_6},
    ops::Div,
    time::Duration,
};

const DEBUG_TEAMMATES: bool = false;
const DEBUG_AIM: bool = false;
const DEBUG_SPREAD: bool = false;
const DEBUG_DETOUR: bool = false;

pub fn on_update(
    mut bots: Query<(&mut Bot, &mut Actor, &Transform, &Kinetics, Option<&Weapon>)>,
    actors: Query<(&Transform, &Kinetics), With<Actor>>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (mut bot, mut actor, transform, kinetics, weapon) in bots.iter_mut() {
        actor.reset_actions();

        let enemy = bot
            .enemy
            .and_then(|e| actors.get(e).ok())
            .map(|e| BotTarget {
                position: e.0.translation.xy(),
                velocity: e.1.velocity,
                direction: e.0.direction(),
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
            velocity: kinetics.velocity,
            weapon,
            spread_out: SpreadOut::Default,
            is_dodging: false,
        };

        if let Some(enemy) = enemy {
            if handler.bot.config.is_agile {
                handler.dodge_enemy(&enemy);
            }

            if !handler.is_dodging {
                handler.attack_enemy(&enemy, time);
            }
        } else {
            handler.idle();
        }

        handler.spread_out(&actors);
    }
}

struct BotHandler<'a> {
    bot: &'a mut Bot,
    actor: &'a mut Actor,
    transform: &'a Transform,
    velocity: Vec2,
    weapon: Option<&'a Weapon>,
    spread_out: SpreadOut,
    is_dodging: bool,
}

impl<'a> BotHandler<'a> {
    fn dodge_enemy(&mut self, enemy: &BotTarget) {
        let bot_to_enemy = self.angle_to(enemy);
        let enemy_to_bot = angle_difference(enemy.direction, enemy.angle_to(self));

        if enemy_to_bot.abs() < BotConfig::DODGE_ANGLE {
            let force = 1.0 - (enemy_to_bot.abs() / BotConfig::DODGE_ANGLE);

            let turn = if enemy_to_bot > 0.0 {
                -FRAC_PI_2
            } else {
                FRAC_PI_2
            };

            if force > 0.5 {
                self.actor.actions |= ActorAction::Sprint;
            }

            self.look_at_direction(bot_to_enemy + turn * force);
            self.actor.movement += Vec2::FRONT;
            self.spread_out.set(SpreadOut::Compact);
            self.is_dodging = true;
        }
    }

    fn attack_enemy(&mut self, enemy: &BotTarget, time: Duration) {
        if let Some(weapon) = self.weapon {
            self.attack_enemy_armed(enemy, weapon, time);
        } else {
            self.attack_enemy_melee(enemy);
        }
    }

    fn attack_enemy_armed(&mut self, target: &BotTarget, weapon: &Weapon, time: Duration) {
        if self.is_close(&target.position, self.bot.config.shoot_distance_min) {
            // don't come too close
            self.actor.movement += Vec2::BACK / 1.5;
            self.spread_out.set(SpreadOut::Disallowed);
        }

        if self.is_reloading() {
            return;
        }

        if self.can_aim_at(target.position) {
            self.spread_out.set(SpreadOut::Simplified);

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
            let is_aimed = self.is_aimed_at_point(target.position);

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
                debug_line(
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

    fn attack_enemy_melee(&mut self, target: &BotTarget) {
        // TODO: count enemy body radius instead of self
        let melee_distance = self.actor.config.melee_distance + self.actor.config.radius;

        if self.is_close(&target.position, melee_distance) {
            // enemy is close, attack
            self.look_at_position(target.position);
            self.actor.actions |= ActorAction::Attack;
            self.actor.movement += Vec2::FRONT;
            self.spread_out.set(SpreadOut::Disallowed);
        } else {
            // otherwise just chase
            self.chase(target);
        }
    }

    #[allow(clippy::needless_late_init)]
    fn chase(&mut self, target: &BotTarget) {
        let meet = self.find_meet(target);
        let target = target.position;
        let detour;

        if self.is_close(&meet, self.bot.config.spread) {
            // meet point is near, no need to spread out and detour
            self.spread_out.set(SpreadOut::Disallowed);
            detour = None;
        } else {
            detour = self
                .bot
                .detour
                .as_ref()
                .and_then(|d| d.calc(self.position(), target));
        }

        if let Some(detour) = detour {
            if self.can_sprint() && self.is_aimed_at_angle(detour) {
                self.actor.actions |= ActorAction::Sprint;
            }

            self.spread_out.set(SpreadOut::Compact);
            self.look_at_direction(detour);
            self.actor.movement += Vec2::FRONT;
        } else {
            if self.can_sprint() && self.is_far(&target, self.bot.config.sprint_distance) {
                // enemy is far, sprint
                self.actor.actions |= ActorAction::Sprint;
            }

            self.look_at_position(meet);
            self.actor.movement += Vec2::FRONT;
        }

        if DEBUG_DETOUR {
            let color = Color::WHITE.with_a(0.4);
            let mut p0 = self.position();
            let mut detour_sign = f32::NAN;

            while let Some(detour) = self.bot.detour.as_ref().and_then(|d| d.calc(p0, target)) {
                let p1 = p0 + Vec2::from_length(0.05, detour);
                debug_line(p0, p1, color);
                p0 = p1;

                if !detour_sign.is_nan() && detour_sign != detour.signum() {
                    break;
                }

                detour_sign = detour.signum();
            }

            debug_line(p0, target, color.with_a(0.1));
        }
    }

    fn idle(&mut self) {
        self.look_at_direction(self.bot.idle_direction);

        if self.bot.idle_movement {
            self.actor.movement += Vec2::FRONT;
        }
    }

    fn spread_out(&mut self, actors: &Query<(&Transform, &Kinetics), With<Actor>>) {
        let mut spread = self.bot.config.spread;
        let is_full;

        match self.spread_out {
            SpreadOut::Default => {
                is_full = true;
            }
            SpreadOut::Compact => {
                spread = 0.0;
                is_full = true;
            }
            SpreadOut::Simplified => {
                is_full = false;
            }
            SpreadOut::Disallowed => {
                return;
            }
        }

        spread = f32::max(spread, self.actor.config.radius * 3.0);

        if DEBUG_SPREAD {
            debug_circle(self.position(), spread, Color::ORANGE.with_a(0.3));
        }

        let mut teammates_position_sum = Vec2::ZERO;
        let mut teammates_position_sum_weight = 0.0;

        for teammate in &self.bot.teammates {
            let Ok(teammate_position) = actors.get(*teammate).map(|a| a.0.translation.xy()) else {
                continue;
            };

            let teammate_distance = self.distance_squared(&teammate_position);

            if teammate_distance < spread * spread {
                let weight = 1.0 - teammate_distance.sqrt() / spread;
                teammates_position_sum += teammate_position * weight;
                teammates_position_sum_weight += weight;
            }
        }

        if teammates_position_sum_weight == 0.0 {
            return;
        }

        let teammates_position = teammates_position_sum / teammates_position_sum_weight;

        if DEBUG_TEAMMATES {
            debug_line(self.position(), teammates_position, Color::GREEN);
        }

        if self.is_close(&teammates_position, spread) {
            if is_full {
                let angle_look = self.get_look_at();
                let angle_to_group = self.angle_to(&teammates_position);
                let angle_distance = angle_difference(angle_look, angle_to_group);

                self.look_at_direction(angle_look - FRAC_PI_2 * angle_distance.signum());

                if angle_distance.abs() > FRAC_PI_6 {
                    self.actor.movement += Vec2::FRONT;
                } else {
                    self.actor.actions -= ActorAction::Sprint;
                }
            } else {
                // find subjective direction to teammates, and go in opposite direction
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

    fn is_aimed_at_point(&self, point: Vec2) -> bool {
        return self.is_aimed_at_angle(self.angle_to(&point));
    }

    fn is_aimed_at_angle(&self, angle: f32) -> bool {
        return angle_difference(self.transform.direction(), angle).abs()
            < self.bot.config.angular_deviation;
    }

    fn is_reloading(&self) -> bool {
        if let Some(weapon) = self.weapon {
            return self.actor.actions.contains(ActorAction::Reload) || weapon.is_reloading();
        } else {
            return false;
        }
    }

    fn can_sprint(&self) -> bool {
        return !self.is_reloading();
    }

    fn can_aim_at(&self, target: Vec2) -> bool {
        return !self.is_reloading() && self.is_close(&target, self.bot.config.shoot_distance_max);
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum SpreadOut {
    Default = 0,
    Compact = 1,
    Simplified = 2,
    Disallowed = 3,
}

impl SpreadOut {
    fn set(&mut self, new: SpreadOut) {
        if (*self as u8) < (new as u8) {
            *self = new;
        }
    }
}

pub struct BotTarget {
    pub position: Vec2,
    pub velocity: Vec2,
    pub direction: f32,
}

impl WithPosition for BotTarget {
    fn position(&self) -> Vec2 {
        return self.position;
    }
}

impl WithVelocity for BotTarget {
    fn velocity(&self) -> Vec2 {
        return self.velocity;
    }
}
