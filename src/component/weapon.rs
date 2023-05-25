use crate::{component::ProjectileConfig, util::ext::DurationExt};
use bevy::ecs::component::Component;
use std::time::Duration;

/// To make sure reloading sounds have stopped
const POST_RELOADING_TIME: Duration = Duration::from_millis(400);

#[derive(Component)]
pub struct Weapon {
    pub config: &'static WeaponConfig,
    is_cocked: bool,
    is_trigger_pressed: bool,
    is_reloading: bool,
    ammo: u8,
    next_time: Duration,
}

#[derive(Clone)]
pub struct WeaponConfig {
    pub name: &'static str,
    pub muzzle_velocity: f32,
    pub fire_rate: f32,
    pub projectile: &'static ProjectileConfig,
    pub ammo_capacity: u8,
    pub reloading_time: Duration,
}

#[allow(dead_code)] // TODO: remove
impl WeaponConfig {
    const SEMI_AUTO_FIRE_RATE: f32 = 400.0;

    const RELOADING_TIME_PISTOL: Duration = Duration::from_millis(3000);
    const RELOADING_TIME_SG: Duration = Duration::from_millis(4000);
    const RELOADING_TIME_RIFLE_LIGHT: Duration = Duration::from_millis(4500);
    const RELOADING_TIME_RIFLE: Duration = Duration::from_millis(5000);
    const RELOADING_TIME_RIFLE_HEAVY: Duration = Duration::from_millis(5500);
    const RELOADING_TIME_MG: Duration = Duration::from_millis(10000);

    pub const LASER_GUN: Self = Self {
        name: "Laser Gun",
        muzzle_velocity: 5000.0,
        fire_rate: 1200.0,
        // radians_deflection: 0,
        // recoil: 0f,
        // is_automatic: true,
        // grip_offset: GRIP_OFFSET_EXTENDED,
        projectile: &ProjectileConfig::LASER,
        ammo_capacity: 0,
        reloading_time: Duration::ZERO,
    };

    pub const PM: Self = Self {
        name: "PM",
        muzzle_velocity: 315.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        // radians_deflection: 0.05,
        // recoil: 6_500,
        // is_automatic: false,
        // grip_offset: GRIP_OFFSET_SHORT,
        projectile: &ProjectileConfig::_9X18,
        ammo_capacity: 8,
        reloading_time: Self::RELOADING_TIME_PISTOL,
    };

    pub const TT: Self = Self {
        name: "TT",
        muzzle_velocity: 430.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        // radians_deflection: 0.05,
        // recoil: 7_500,
        // is_automatic: false,
        // grip_offset: GRIP_OFFSET_SHORT,
        projectile: &ProjectileConfig::_7_62X25,
        ammo_capacity: 8,
        reloading_time: Self::RELOADING_TIME_PISTOL,
    };

    pub const MP_43_SAWED_OFF: Self = Self {
        name: "MP-43 sawed-off",
        muzzle_velocity: 260.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        // radians_deflection: 0.08,
        // recoil: 45_000,
        // is_automatic: false,
        // grip_offset: GRIP_OFFSET_STEP * 4,
        projectile: &ProjectileConfig::_12X76,
        ammo_capacity: 2,
        reloading_time: Self::RELOADING_TIME_RIFLE_LIGHT,
    };

    pub const MP_27: Self = Self {
        name: "MP-27",
        muzzle_velocity: 410.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        // radians_deflection: 0.06,
        // recoil: 38_000,
        // is_automatic: false,
        // grip_offset: GRIP_OFFSET_EXTENDED,
        projectile: &ProjectileConfig::_12X76,
        ammo_capacity: 2,
        reloading_time: Self::RELOADING_TIME_RIFLE,
    };

    pub const PP_91_KEDR: Self = Self {
        name: "PP-91 Kedr",
        muzzle_velocity: 310.0,
        fire_rate: 900.0,
        // radians_deflection: 0.04,
        // recoil: 7_000,
        // is_automatic: true,
        // grip_offset: GRIP_OFFSET_COMMON,
        projectile: &ProjectileConfig::_9X18,
        ammo_capacity: 20,
        reloading_time: Self::RELOADING_TIME_SG,
    };

    pub const PP_19_BIZON: Self = Self {
        name: "PP-19 Bizon",
        muzzle_velocity: 330.0,
        fire_rate: 680.0,
        // radians_deflection: 0.03,
        // recoil: 7_500,
        // is_automatic: true,
        // grip_offset: GRIP_OFFSET_COMMON,
        projectile: &ProjectileConfig::_9X18,
        ammo_capacity: 64,
        reloading_time: Self::RELOADING_TIME_SG,
    };

    pub const AKS_74U: Self = Self {
        name: "AKS-74U",
        muzzle_velocity: 735.0,
        fire_rate: 675.0,
        // radians_deflection: 0.03,
        // recoil: 12_000,
        // is_automatic: true,
        // grip_offset: GRIP_OFFSET_COMMON,
        projectile: &ProjectileConfig::_5_45X39,
        ammo_capacity: 30,
        reloading_time: Self::RELOADING_TIME_RIFLE_LIGHT,
    };

    pub const AK_74M: Self = Self {
        name: "AK-74M",
        muzzle_velocity: 910.0,
        fire_rate: 600.0,
        // radians_deflection: 0.028,
        // recoil: 14_000,
        // is_automatic: true,
        // grip_offset: GRIP_OFFSET_COMMON,
        projectile: &ProjectileConfig::_5_45X39,
        ammo_capacity: 30,
        reloading_time: Self::RELOADING_TIME_RIFLE,
    };

    pub const RPK_74: Self = Self {
        name: "RPK-74",
        muzzle_velocity: 960.0,
        fire_rate: 600.0,
        // radians_deflection: 0.025,
        // recoil: 19_000,
        // is_automatic: true,
        // grip_offset: GRIP_OFFSET_COMMON,
        projectile: &ProjectileConfig::_5_45X39,
        ammo_capacity: 45,
        reloading_time: Self::RELOADING_TIME_RIFLE_HEAVY,
    };

    pub const SAIGA_12K: Self = Self {
        name: "Saiga-12K",
        muzzle_velocity: 410.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        // radians_deflection: 0.07,
        // recoil: 32_000,
        // is_automatic: false,
        // grip_offset: GRIP_OFFSET_COMMON,
        projectile: &ProjectileConfig::_12X76,
        ammo_capacity: 8,
        reloading_time: Self::RELOADING_TIME_RIFLE,
    };

    pub const PKM: Self = Self {
        name: "PKM",
        muzzle_velocity: 825.0,
        fire_rate: 650.0,
        // radians_deflection: 0.021,
        // recoil: 22_000,
        // is_automatic: true,
        // grip_offset: GRIP_OFFSET_EXTENDED,
        projectile: &ProjectileConfig::_7_62X54,
        ammo_capacity: 100,
        reloading_time: Self::RELOADING_TIME_MG,
    };

    pub const PKP_PECHENEG: Self = Self {
        name: "PKP Pecheneg",
        muzzle_velocity: 825.0,
        fire_rate: 650.0,
        // radians_deflection: 0.02,
        // recoil: 22_000,
        // is_automatic: true,
        // grip_offset: GRIP_OFFSET_EXTENDED,
        projectile: &ProjectileConfig::_7_62X54,
        ammo_capacity: 100,
        reloading_time: Self::RELOADING_TIME_MG,
    };
}

impl Weapon {
    pub const fn new(config: &'static WeaponConfig) -> Self {
        return Self {
            config,
            is_cocked: true,
            is_trigger_pressed: false,
            is_reloading: false,
            ammo: config.ammo_capacity,
            next_time: Duration::from_secs(0),
        };
    }

    pub fn fire(&mut self, time: Duration) -> WeaponFireResult {
        self.is_trigger_pressed = true;

        if self.is_ready(time) {
            self.next_time = time + Duration::from_secs_f32(60.0 / self.config.fire_rate);

            if self.ammo > 0 {
                self.ammo -= 1;
                return WeaponFireResult::Fire;
            } else {
                self.is_cocked = false;
                return WeaponFireResult::Empty;
            }
        } else {
            return WeaponFireResult::NotReady;
        }
    }

    pub fn release_trigger(&mut self) {
        self.is_trigger_pressed = false;
    }

    pub fn reload(&mut self, time: Duration) {
        if !self.is_reloading {
            self.is_reloading = true;
            self.next_time = time + self.config.reloading_time;
        }
    }

    pub fn complete_reloading(&mut self, time: Duration) {
        if self.is_reloading {
            self.is_cocked = true;
            self.is_reloading = false;
            self.ammo = self.config.ammo_capacity;
            self.next_time = time + POST_RELOADING_TIME;
        }
    }

    pub fn get_ammo_normalized(&self, time: Duration) -> f32 {
        if self.is_reloading {
            return time.get_progress(
                self.next_time.saturating_sub(self.config.reloading_time),
                self.next_time,
            );
        } else {
            if self.config.ammo_capacity == 0 {
                return 1.0;
            } else {
                return f32::from(self.ammo) / f32::from(self.config.ammo_capacity);
            }
        }
    }

    pub fn is_cocked(&self) -> bool {
        return self.is_cocked;
    }

    pub fn is_trigger_pressed(&self) -> bool {
        return self.is_trigger_pressed;
    }

    pub fn is_reloading(&self) -> bool {
        return self.is_reloading;
    }

    pub fn is_ready(&self, time: Duration) -> bool {
        return self.next_time < time;
    }

    pub fn is_ammo_full(&self) -> bool {
        return self.ammo == self.config.ammo_capacity;
    }
}

pub enum WeaponFireResult {
    NotReady,
    Empty,
    Fire,
}
