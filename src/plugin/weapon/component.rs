use crate::{plugin::WeaponConfig, util::ext::DurationExt};
use bevy::ecs::component::Component;
use std::time::Duration;

const ARMING_DURATION: Duration = Duration::from_millis(150);

#[derive(Component)]
pub struct Weapon {
    pub config: &'static WeaponConfig,
    ammo: u8,
    deviation_temporal: f32,
    reloading: Option<Duration>,
    last_shot: Duration,
    next_time: Duration,
}

impl Weapon {
    pub const BARREL_LENGTH: f32 = 0.6; // TODO: don't hardcode

    pub const fn new(config: &'static WeaponConfig) -> Self {
        return Self {
            config,
            ammo: config.ammo_capacity,
            deviation_temporal: 0.0,
            reloading: None,
            last_shot: Duration::ZERO,
            next_time: Duration::ZERO,
        };
    }

    pub fn try_fire(&mut self, time: Duration) -> bool {
        if self.is_ready(time) && self.has_ammo() {
            let deviation = self.get_deviation_temporal(time);
            self.ammo = self.ammo.saturating_sub(1);
            self.last_shot = time;
            self.next_time = time + Duration::from_secs_f32(60.0 / self.config.fire_rate);
            self.deviation_temporal = self.config.deviation + deviation;
            return true;
        } else {
            return false;
        }
    }

    pub fn reload(&mut self, time: Duration, duration: Duration) {
        if self.reloading.is_none() {
            self.reloading = Some(duration);
            self.next_time = time + duration;
        }
    }

    pub fn complete_reloading(&mut self, time: Duration) {
        if self.reloading.is_some() {
            let was_armed = self.is_armed();
            self.reloading = None;
            self.ammo = self.config.ammo_capacity;

            if !was_armed {
                self.next_time = time + ARMING_DURATION;
            }
        }
    }

    pub fn get_deviation_temporal(&self, time: Duration) -> f32 {
        let accuracy = time.progress(
            self.last_shot,
            self.last_shot + WeaponConfig::DEVIATION_COOL_DOWN,
        );

        return self.deviation_temporal * (1.0 - accuracy);
    }

    pub fn get_deviation(&self, time: Duration) -> f32 {
        return self.config.deviation + self.get_deviation_temporal(time);
    }

    pub fn get_mass(&self) -> f32 {
        return self.config.mass + self.config.projectile.mass * f32::from(self.ammo);
    }

    pub fn get_recoil(&self) -> f32 {
        let momentum = self.config.muzzle_velocity * self.config.projectile.mass;
        let mass = self.get_mass().powf(WeaponConfig::RECOIL_MASS_POW);

        return (momentum / mass).powf(WeaponConfig::RECOIL_POW)
            * WeaponConfig::RECOIL_MUL
            * self.config.grip.recoil_factor();
    }

    pub fn get_ammo_normalized(&self, time: Duration) -> f32 {
        if let Some(reloading_duration) = self.reloading {
            let progress = time.progress(
                self.next_time.saturating_sub(reloading_duration),
                self.next_time,
            );

            return progress;
        } else {
            return self.config.get_ammo_normalized(self.ammo);
        }
    }

    pub fn has_ammo(&self) -> bool {
        return self.ammo > 0 || self.config.ammo_capacity == 0;
    }

    pub fn is_armed(&self) -> bool {
        return !self.config.has_bolt || self.has_ammo();
    }

    pub fn is_reloading(&self) -> bool {
        return self.reloading.is_some();
    }

    pub fn is_ready(&self, time: Duration) -> bool {
        return self.next_time < time;
    }
}
