use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::math::Vec2;
use std::time::Duration;

const VISUAL_VELOCITY_FACTOR: f32 = 1.0 / 5.0;

#[derive(Component)]
pub struct Projectile {
    pub config: ProjectileConfig,
    pub initial_time: Duration,
    pub initial_position: Vec2,
    pub initial_velocity: Vec2,
    pub shooter: Option<Entity>,
    pub stopped: bool,
}

#[derive(Clone)]
pub struct ProjectileConfig {
    pub acceleration_factor: f32,
}

impl Projectile {
    pub const MASS: f32 = 8.0;
    pub const PUSH_FACTOR: f32 = 30.0;
    pub const PUSH_FACTOR_ANGULAR: f32 = 200.0;
    pub const VELOCITY_MIN: f32 = 5.0;

    pub const fn new(
        config: ProjectileConfig,
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
        let a = self.config.acceleration_factor;
        let p = self.initial_position;
        let v0 = self.initial_velocity;
        let v1 = v0 * (t * a).exp();
        return (p + (v1 - v0) / a * VISUAL_VELOCITY_FACTOR, v1);
    }
}
