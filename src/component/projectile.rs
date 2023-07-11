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

#[derive(Clone)]
pub struct ProjectileConfig {
    pub fragments: u8,
    pub mass: f32,
    pub size: f32,
}

impl ProjectileConfig {
    pub const _9X18: Self = Self {
        fragments: 1,
        mass: 0.0061,
        size: 0.7,
    };

    pub const _7_62X25: Self = Self {
        fragments: 1,
        mass: 0.0055,
        size: 0.7,
    };

    pub const _12X76: Self = Self {
        fragments: 12,
        mass: 0.048,
        size: 0.1,
    };

    pub const _5_45X39: Self = Self {
        fragments: 1,
        mass: 0.0034,
        size: 1.0,
    };

    pub const _7_62X54: Self = Self {
        fragments: 1,
        mass: 0.0096,
        size: 1.2,
    };

    pub fn acceleration(&self) -> f32 {
        return -1.0 / self.fragment_mass() * 0.006 - 4.2;
    }

    pub fn fragment_mass(&self) -> f32 {
        return self.mass / f32::from(self.fragments);
    }
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
