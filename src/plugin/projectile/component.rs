use crate::plugin::ProjectileConfig;
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
        let a = self.config.acceleration();
        let p = self.initial_position;
        let v0 = self.initial_velocity;
        let v1 = v0 * (t * a).exp();
        return (p + (v1 - v0) / a * Projectile::VELOCITY_VISUAL_FACTOR, v1);
    }
}
