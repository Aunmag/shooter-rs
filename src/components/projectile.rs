use crate::utils::DurationExt;
use amethyst::core::math::Vector2;
use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;
use amethyst::ecs::Entity;
use std::time::Duration;

const VISUAL_VELOCITY_FACTOR: f32 = 1.0 / 5.0;

pub struct Projectile {
    pub config: ProjectileConfig,
    pub initial_time: Duration,
    pub initial_position: Vector2<f32>,
    pub initial_velocity: Vector2<f32>,
    pub shooter: Option<Entity>,
}

#[derive(Clone)]
pub struct ProjectileConfig {
    pub acceleration_factor: f32,
}

impl Projectile {
    pub const MASS: f32 = 8.0;
    pub const PUSH_FACTOR: f32 = 30.0;
    pub const VELOCITY_MIN: f32 = 5.0;

    pub const fn new(
        config: ProjectileConfig,
        time: Duration,
        position: Vector2<f32>,
        velocity: Vector2<f32>,
        shooter: Option<Entity>,
    ) -> Self {
        return Self {
            config,
            initial_time: time,
            initial_position: position,
            initial_velocity: velocity,
            shooter,
        };
    }

    pub fn calc_data(&self, time: Duration) -> (Vector2<f32>, Vector2<f32>) {
        let t = time.sub_safely(self.initial_time).as_secs_f32();
        let a = self.config.acceleration_factor;
        let p = &self.initial_position;
        let v_0 = &self.initial_velocity;
        let v_1 = v_0 * (t * a).exp();

        return (p + (v_1 - v_0) / a * VISUAL_VELOCITY_FACTOR, v_1);
    }
}

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;
}
