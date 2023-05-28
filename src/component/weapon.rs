use crate::{
    component::ProjectileConfig,
    util::{
        ext::{DurationExt, Pcg32Ext},
        math::interpolate,
    },
};
use bevy::ecs::component::Component;
use rand_pcg::Pcg32;
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

pub struct WeaponConfig {
    pub name: &'static str,
    pub level: u8,
    pub muzzle_velocity: f32,
    pub deviation: f32,
    pub recoil: f32,
    pub fire_rate: f32,
    pub is_automatic: bool,
    pub projectile: &'static ProjectileConfig,
    pub ammo_capacity: u8,
    pub reloading_time: Duration,
    pub partial_reloading: bool,
    pub image_offset: f32,
    pub actor_image_suffix: u8, // TODO: detect automatically by length
}

impl WeaponConfig {
    const VELOCITY_DEVIATION: f32 = 0.06;

    const SEMI_AUTO_FIRE_RATE: f32 = 400.0;

    /// To make game easier modify real reloading time
    const RELOADING_TIME_FACTOR: f32 = 0.65;

    const RELOADING_TIME_PISTOL: Duration =
        Duration::from_millis((3000.0 * Self::RELOADING_TIME_FACTOR) as u64);

    const RELOADING_TIME_SG: Duration =
        Duration::from_millis((1800.0 * Self::RELOADING_TIME_FACTOR) as u64);

    const RELOADING_TIME_RIFLE_LIGHT: Duration =
        Duration::from_millis((4500.0 * Self::RELOADING_TIME_FACTOR) as u64);

    const RELOADING_TIME_RIFLE: Duration =
        Duration::from_millis((5000.0 * Self::RELOADING_TIME_FACTOR) as u64);

    const RELOADING_TIME_RIFLE_HEAVY: Duration =
        Duration::from_millis((5500.0 * Self::RELOADING_TIME_FACTOR) as u64);

    const RELOADING_TIME_MG: Duration =
        Duration::from_millis((10000.0 * Self::RELOADING_TIME_FACTOR) as u64);

    pub const ALL: [Self; 12] = [
        Self::PM,
        Self::TT,
        Self::MP_43_SAWED_OFF,
        Self::MP_27,
        Self::PP_91_KEDR,
        Self::PP_19_BIZON,
        Self::AKS_74U,
        Self::AK_74M,
        Self::RPK_74,
        Self::SAIGA_12K,
        Self::PKM,
        Self::PKP_PECHENEG,
    ];

    pub const PM: Self = Self {
        name: "PM",
        level: 1,
        muzzle_velocity: 315.0,
        deviation: 0.03,
        recoil: 9_750.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        is_automatic: false,
        projectile: &ProjectileConfig::_9X18,
        ammo_capacity: 8,
        reloading_time: Self::RELOADING_TIME_PISTOL,
        partial_reloading: false,
        image_offset: 2.0,
        actor_image_suffix: 1,
    };

    pub const TT: Self = Self {
        name: "TT",
        level: 1,
        muzzle_velocity: 430.0,
        deviation: 0.025,
        recoil: 11_250.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        is_automatic: false,
        projectile: &ProjectileConfig::_7_62X25,
        ammo_capacity: 8,
        reloading_time: Self::RELOADING_TIME_PISTOL,
        partial_reloading: false,
        image_offset: 2.0,
        actor_image_suffix: 1,
    };

    pub const MP_43_SAWED_OFF: Self = Self {
        name: "MP-43 sawed-off",
        level: 2,
        muzzle_velocity: 260.0,
        deviation: 0.04,
        recoil: 67_500.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        is_automatic: false,
        projectile: &ProjectileConfig::_12X76,
        ammo_capacity: 2,
        reloading_time: Self::RELOADING_TIME_RIFLE_LIGHT,
        partial_reloading: true,
        image_offset: 3.5,
        actor_image_suffix: 2,
    };

    pub const MP_27: Self = Self {
        name: "MP-27",
        level: 2,
        muzzle_velocity: 410.0,
        deviation: 0.03,
        recoil: 57_000.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        is_automatic: false,
        projectile: &ProjectileConfig::_12X76,
        ammo_capacity: 2,
        reloading_time: Self::RELOADING_TIME_RIFLE,
        partial_reloading: true,
        image_offset: 10.0,
        actor_image_suffix: 2,
    };

    pub const PP_91_KEDR: Self = Self {
        name: "PP-91 Kedr",
        level: 3,
        muzzle_velocity: 310.0,
        deviation: 0.02,
        recoil: 10_500.0,
        fire_rate: 900.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_9X18,
        ammo_capacity: 20,
        reloading_time: Self::RELOADING_TIME_SG,
        partial_reloading: false,
        image_offset: 3.5,
        actor_image_suffix: 2,
    };

    pub const PP_19_BIZON: Self = Self {
        name: "PP-19 Bizon",
        level: 3,
        muzzle_velocity: 330.0,
        deviation: 0.015,
        recoil: 11_250.0,
        fire_rate: 680.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_9X18,
        ammo_capacity: 64,
        reloading_time: Self::RELOADING_TIME_SG,
        partial_reloading: false,
        image_offset: 7.0,
        actor_image_suffix: 2,
    };

    pub const AKS_74U: Self = Self {
        name: "AKS-74U",
        level: 4,
        muzzle_velocity: 735.0,
        deviation: 0.015,
        recoil: 18_000.0,
        fire_rate: 675.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_5_45X39,
        ammo_capacity: 30,
        reloading_time: Self::RELOADING_TIME_RIFLE_LIGHT,
        partial_reloading: false,
        image_offset: 8.0,
        actor_image_suffix: 2,
    };

    pub const AK_74M: Self = Self {
        name: "AK-74M",
        level: 4,
        muzzle_velocity: 910.0,
        deviation: 0.014,
        recoil: 21_000.0,
        fire_rate: 600.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_5_45X39,
        ammo_capacity: 30,
        reloading_time: Self::RELOADING_TIME_RIFLE,
        partial_reloading: false,
        image_offset: 9.0,
        actor_image_suffix: 2,
    };

    pub const RPK_74: Self = Self {
        name: "RPK-74",
        level: 5,
        muzzle_velocity: 960.0,
        deviation: 0.012,
        recoil: 28_500.0,
        fire_rate: 600.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_5_45X39,
        ammo_capacity: 45,
        reloading_time: Self::RELOADING_TIME_RIFLE_HEAVY,
        partial_reloading: false,
        image_offset: 9.0,
        actor_image_suffix: 2,
    };

    pub const SAIGA_12K: Self = Self {
        name: "Saiga-12K",
        level: 5,
        muzzle_velocity: 410.0,
        deviation: 0.035,
        recoil: 48_000.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        is_automatic: false,
        projectile: &ProjectileConfig::_12X76,
        ammo_capacity: 8,
        reloading_time: Self::RELOADING_TIME_RIFLE,
        partial_reloading: false,
        image_offset: 9.0,
        actor_image_suffix: 2,
    };

    pub const PKM: Self = Self {
        name: "PKM",
        level: 6,
        muzzle_velocity: 825.0,
        deviation: 0.011,
        recoil: 33_000.0,
        fire_rate: 650.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_7_62X54,
        ammo_capacity: 100,
        reloading_time: Self::RELOADING_TIME_MG,
        partial_reloading: false,
        image_offset: 10.0,
        actor_image_suffix: 2,
    };

    pub const PKP_PECHENEG: Self = Self {
        name: "PKP Pecheneg",
        level: 6,
        muzzle_velocity: 825.0,
        deviation: 0.01,
        recoil: 33_000.0,
        fire_rate: 650.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_7_62X54,
        ammo_capacity: 100,
        reloading_time: Self::RELOADING_TIME_MG,
        partial_reloading: false,
        image_offset: 10.0,
        actor_image_suffix: 2,
    };

    pub fn generate_velocity(&self, rng: &mut Pcg32) -> f32 {
        let deviation = rng.gen_normal(self.muzzle_velocity * Self::VELOCITY_DEVIATION);
        return self.muzzle_velocity + deviation;
    }

    pub fn generate_deviation(&self, rng: &mut Pcg32) -> f32 {
        return rng.gen_normal(self.deviation);
    }

    pub fn get_image_path(&self) -> String {
        return format!("weapons/{}/image.png", self.name);
    }

    pub fn get_ammo_normalized(&self, ammo: u8) -> f32 {
        if self.ammo_capacity == 0 {
            return 1.0;
        } else {
            return f32::from(ammo) / f32::from(self.ammo_capacity);
        }
    }
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
        if !self.config.is_automatic && self.is_trigger_pressed {
            return WeaponFireResult::NotReady;
        }

        self.is_trigger_pressed = true;

        if self.is_ready(time) {
            self.next_time = time + Duration::from_secs_f32(60.0 / self.config.fire_rate);

            if self.config.ammo_capacity == 0 {
                return WeaponFireResult::Fire;
            } else if self.ammo > 0 {
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
            if self.config.partial_reloading {
                if self.ammo == self.config.ammo_capacity {
                    self.ammo = self.ammo.saturating_sub(1);
                }
            } else {
                self.ammo = 0;
            }

            self.is_reloading = true;
            self.next_time = time + self.config.reloading_time;
        }
    }

    pub fn complete_reloading(&mut self, time: Duration) {
        if self.is_reloading {
            self.is_cocked = true;
            self.is_reloading = false;

            if self.config.partial_reloading {
                if self.ammo < self.config.ammo_capacity {
                    self.ammo += 1;
                }
            } else {
                self.ammo = self.config.ammo_capacity;
            }

            self.next_time = time + POST_RELOADING_TIME;
        }
    }

    pub fn get_ammo_normalized(&self, time: Duration) -> f32 {
        if self.is_reloading {
            let progress = time.progress(
                self.next_time.saturating_sub(self.config.reloading_time),
                self.next_time,
            );

            if self.config.partial_reloading {
                return interpolate(
                    self.config.get_ammo_normalized(self.ammo),
                    self.config.get_ammo_normalized(self.ammo + 1),
                    progress,
                );
            } else {
                return progress;
            }
        } else {
            return self.config.get_ammo_normalized(self.ammo);
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
}

pub enum WeaponFireResult {
    NotReady,
    Empty,
    Fire,
}
