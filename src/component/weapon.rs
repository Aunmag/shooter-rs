use crate::component::ProjectileConfig;
use bevy::ecs::component::Component;
use std::time::Duration;

#[derive(Component)]
pub struct Weapon {
    pub config: WeaponConfig,
    next_shoot_time: Duration,
}

#[derive(Clone)]
pub struct WeaponConfig {
    pub muzzle_velocity: f32,
    pub fire_rate: f32,
    pub projectile: ProjectileConfig,
}

impl Weapon {
    pub const fn new(config: WeaponConfig) -> Self {
        return Self {
            config,
            next_shoot_time: Duration::from_secs(0),
        };
    }

    pub fn fire(&mut self, time: Duration) -> bool {
        if time >= self.next_shoot_time {
            self.next_shoot_time = time + Duration::from_secs_f32(60.0 / self.config.fire_rate);
            return true;
        } else {
            return false;
        }
    }
}
