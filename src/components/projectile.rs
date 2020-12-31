use crate::utils;
use amethyst::core::math::Point2;
use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;
use std::f32::consts::E;
use std::time::Duration;

const VELOCITY_MIN: f32 = 5.0;
const VISUAL_VELOCITY_FACTOR: f32 = 1.0 / 5.0;

pub struct Projectile {
    pub config: ProjectileConfig,
    pub initial_time: Duration,
    pub initial_position: Point2<f32>,
    pub initial_velocity: Point2<f32>,
}

#[derive(Clone)]
pub struct ProjectileConfig {
    pub acceleration_factor: f32,
}

pub struct ProjectileData {
    pub head: Point2<f32>,
    pub tail: Point2<f32>,
    pub velocity: Point2<f32>,
}

impl Projectile {
    pub fn new(
        config: ProjectileConfig,
        time: Duration,
        position: Point2<f32>,
        velocity: Point2<f32>,
    ) -> Self {
        return Self {
            config,
            initial_time: time,
            initial_position: position,
            initial_velocity: velocity,
        };
    }

    pub fn calc_data(&self, time: Duration, delta: f32) -> ProjectileData {
        let elapsed = (time - self.initial_time).as_secs_f32();
        let acceleration = E.powf(self.config.acceleration_factor * elapsed);

        let velocity = Point2::from([
            self.initial_velocity.x * acceleration,
            self.initial_velocity.y * acceleration,
        ]);

        let traveled_x = (velocity.x - self.initial_velocity.x) / self.config.acceleration_factor;
        let traveled_y = (velocity.y - self.initial_velocity.y) / self.config.acceleration_factor;

        let tail = Point2::from([
            self.initial_position.x + traveled_x * VISUAL_VELOCITY_FACTOR,
            self.initial_position.y + traveled_y * VISUAL_VELOCITY_FACTOR,
        ]);

        let head = Point2::from([
            tail.x + velocity.x * VISUAL_VELOCITY_FACTOR * delta,
            tail.y + velocity.y * VISUAL_VELOCITY_FACTOR * delta,
        ]);

        return ProjectileData {
            head,
            tail,
            velocity,
        };
    }
}

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;
}

impl ProjectileData {
    pub fn has_stopped(&self) -> bool {
        return utils::math::are_closer_than(
            self.velocity.x,
            self.velocity.y,
            0.0,
            0.0,
            VELOCITY_MIN,
        );
    }
}
