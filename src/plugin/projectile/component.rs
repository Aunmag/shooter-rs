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
    pub distance_limit: f32,
    pub shooter: Option<Entity>,
    pub stopped: bool,
}

impl Projectile {
    pub const fn new(
        config: &'static ProjectileConfig,
        time: Duration,
        position: Vec2,
        velocity: Vec2,
        distance_limit: f32,
        shooter: Option<Entity>,
    ) -> Self {
        return Self {
            config,
            initial_time: time,
            initial_position: position,
            initial_velocity: velocity,
            distance_limit: distance_limit.clamp(0.0, config.physics.distance_limit()),
            shooter,
            stopped: false,
        };
    }
}
