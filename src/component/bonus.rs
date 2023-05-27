use super::WeaponConfig;
use bevy::ecs::component::Component;
use std::time::Duration;

#[derive(Component)]
pub struct Bonus {
    pub weapon: &'static WeaponConfig,
    expiration: Duration,
}

impl Bonus {
    pub const RADIUS: f32 = 0.2;
    pub const PULSE: Duration = Duration::from_secs(2);
    pub const TEXT_SCALE_MIN: f32 = 0.39;
    pub const TEXT_SCALE_MAX: f32 = 0.41;
    pub const LIFETIME: Duration = Duration::from_secs(30);

    pub fn new(weapon: &'static WeaponConfig, time: Duration) -> Self {
        return Self {
            weapon,
            expiration: time + Self::LIFETIME,
        };
    }

    pub fn is_expired(&self, time: Duration) -> bool {
        return time > self.expiration;
    }
}

#[derive(Component)]
pub struct BonusImage;

#[derive(Component)]
pub struct BonusLabel;
