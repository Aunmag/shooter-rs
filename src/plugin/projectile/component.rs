use crate::{plugin::ProjectileConfig, util::ext::Vec2Ext};
use bevy::{
    ecs::{component::Component, entity::Entity},
    math::Vec2,
};
use std::time::Duration;

#[derive(Component)]
pub struct Projectile {
    pub config: &'static ProjectileConfig,
    pub initial_time: Duration,
    pub initial_position: Vec2,
    pub initial_velocity: Vec2,
    pub shooter: Option<Entity>,
    pub stopped: bool,
}

impl Projectile {
    pub const VELOCITY_MIN: f32 = 5.0;
    pub const VELOCITY_VISUAL_FACTOR: f32 = 1.0 / 5.0;
    pub const ROCKET_TRAVEL_LIMIT: f32 = 40.0;

    pub const fn new(
        config: &'static ProjectileConfig,
        time: Duration,
        position: Vec2,
        velocity: Vec2,
        shooter: Option<Entity>,
    ) -> Self {
        return Self {
            config,
            initial_time: time,
            initial_position: position,
            initial_velocity: velocity,
            shooter,
            stopped: false,
        };
    }

    pub fn calc_data(&self, time: Duration) -> (Vec2, Vec2) {
        let t = time.saturating_sub(self.initial_time).as_secs_f32();

        if self.config.is_rocket {
            let a = Duration::from_secs_f32(0.8).as_secs_f32(); // acceleration
            let v1 = self.initial_velocity / 8.0;
            let v2 = self.initial_velocity;
            let progress = (t / a).clamp(0.0, 1.0);

            let traveled = Self::VELOCITY_VISUAL_FACTOR
                * if progress < 1.0 {
                    v1 * t + ((v2 - v1) / (2.0 * a) * t.powi(2))
                } else {
                    v2 * t - ((v2 - v1) / 2.0 * a)
                };

            let velocity = if traveled.is_long(Self::ROCKET_TRAVEL_LIMIT) {
                Vec2::ZERO // just to explode
            } else {
                v1 + (v2 - v1) * progress
            };

            return (self.initial_position + traveled, velocity);
        }

        let a = self.config.acceleration(); // acceleration
        let v1 = self.initial_velocity;
        let v2 = v1 * (t * a).exp();
        let traveled = (v2 - v1) / a * Self::VELOCITY_VISUAL_FACTOR;

        return (self.initial_position + traveled, v2);
    }
}
