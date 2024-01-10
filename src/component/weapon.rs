use crate::{
    component::ProjectileConfig,
    util::ext::{DurationExt, RngExt},
};
use bevy::ecs::component::Component;
use rand_pcg::Pcg32;
use std::time::Duration;

const ARMING_DURATION: Duration = Duration::from_millis(300);

#[derive(Component)]
pub struct Weapon {
    pub config: &'static WeaponConfig,
    reloading: Option<Duration>,
    ammo: u8,
    next_time: Duration,
}

pub struct WeaponConfig {
    pub name: &'static str,
    pub level: u8,
    pub mass: f32,
    pub muzzle_velocity: f32,
    pub deviation: f32,
    pub fire_rate: f32,
    pub is_automatic: bool,
    pub projectile: &'static ProjectileConfig,
    pub ammo_capacity: u8,
    pub reloading_time: Duration,
    pub has_bolt: bool,
    pub grip: WeaponGrip,
    pub image_offset: f32,
}

impl WeaponConfig {
    const VELOCITY_DEVIATION: f32 = 0.06;
    const FIRE_RATE_SHOTGUN: f32 = 85.0;

    const RELOADING_TIME_PISTOL: Duration = Duration::from_millis(3000);
    const RELOADING_TIME_SHOTGUN_LIGHT: Duration = Duration::from_millis(1200);
    const RELOADING_TIME_SHOTGUN: Duration = Duration::from_millis(1800);
    const RELOADING_TIME_SMG: Duration = Duration::from_millis(3800);
    const RELOADING_TIME_CARBINE: Duration = Duration::from_millis(4200);
    const RELOADING_TIME_RIFLE: Duration = Duration::from_millis(5000);
    const RELOADING_TIME_RIFLE_HEAVY: Duration = Duration::from_millis(5800);
    const RELOADING_TIME_MACHINE_GUN: Duration = Duration::from_millis(10000);

    const RECOIL_MASS_POW: f32 = 0.25;
    const RECOIL_POW: f32 = 0.5;
    const RECOIL_MUL: f32 = 13.0;

    pub const ALL: [Self; 12] = [
        Self::PM,
        Self::TT,
        Self::MP_43_SAWED_OFF,
        Self::PP_91_KEDR,
        Self::MP_27,
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
        mass: 0.73,
        muzzle_velocity: 315.0,
        deviation: 0.03,
        fire_rate: 120.0,
        is_automatic: false,
        projectile: &ProjectileConfig::_9X18,
        ammo_capacity: 8,
        reloading_time: Self::RELOADING_TIME_PISTOL,
        has_bolt: true,
        grip: WeaponGrip::OneHand,
        image_offset: 2.0,
    };

    pub const TT: Self = Self {
        name: "TT",
        level: 1,
        mass: 0.85,
        muzzle_velocity: 430.0,
        deviation: 0.025,
        fire_rate: 110.0,
        is_automatic: false,
        projectile: &ProjectileConfig::_7_62X25,
        ammo_capacity: 8,
        reloading_time: Self::RELOADING_TIME_PISTOL,
        has_bolt: true,
        grip: WeaponGrip::OneHand,
        image_offset: 2.0,
    };

    pub const MP_43_SAWED_OFF: Self = Self {
        name: "MP-43 sawed-off",
        level: 2,
        mass: 2.2,
        muzzle_velocity: 260.0,
        deviation: 0.06,
        fire_rate: Self::FIRE_RATE_SHOTGUN,
        is_automatic: false,
        projectile: &ProjectileConfig::_12X76,
        ammo_capacity: 2,
        reloading_time: Self::RELOADING_TIME_SHOTGUN_LIGHT,
        has_bolt: false,
        grip: WeaponGrip::TwoHands,
        image_offset: 3.5,
    };

    pub const PP_91_KEDR: Self = Self {
        name: "PP-91 Kedr",
        level: 2,
        mass: 1.6,
        muzzle_velocity: 310.0,
        deviation: 0.02,
        fire_rate: 900.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_9X18,
        ammo_capacity: 20,
        reloading_time: Self::RELOADING_TIME_SMG,
        has_bolt: true,
        grip: WeaponGrip::TwoHands,
        image_offset: 3.5,
    };

    pub const MP_27: Self = Self {
        name: "MP-27",
        level: 3,
        mass: 3.2,
        muzzle_velocity: 410.0,
        deviation: 0.03,
        fire_rate: Self::FIRE_RATE_SHOTGUN,
        is_automatic: false,
        projectile: &ProjectileConfig::_12X76,
        ammo_capacity: 2,
        reloading_time: Self::RELOADING_TIME_SHOTGUN,
        has_bolt: false,
        grip: WeaponGrip::TwoHandsWithButt,
        image_offset: 10.0,
    };

    pub const PP_19_BIZON: Self = Self {
        name: "PP-19 Bizon",
        level: 3,
        mass: 2.7,
        muzzle_velocity: 330.0,
        deviation: 0.015,
        fire_rate: 680.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_9X18,
        ammo_capacity: 64,
        reloading_time: Self::RELOADING_TIME_SMG,
        has_bolt: true,
        grip: WeaponGrip::TwoHandsWithButt,
        image_offset: 7.0,
    };

    pub const AKS_74U: Self = Self {
        name: "AKS-74U",
        level: 4,
        mass: 2.9,
        muzzle_velocity: 735.0,
        deviation: 0.015,
        fire_rate: 675.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_5_45X39,
        ammo_capacity: 30,
        reloading_time: Self::RELOADING_TIME_CARBINE,
        has_bolt: true,
        grip: WeaponGrip::TwoHandsWithButt,
        image_offset: 8.0,
    };

    pub const AK_74M: Self = Self {
        name: "AK-74M",
        level: 4,
        mass: 3.83,
        muzzle_velocity: 910.0,
        deviation: 0.014,
        fire_rate: 600.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_5_45X39,
        ammo_capacity: 30,
        reloading_time: Self::RELOADING_TIME_RIFLE,
        has_bolt: true,
        grip: WeaponGrip::TwoHandsWithButt,
        image_offset: 9.0,
    };

    pub const RPK_74: Self = Self {
        name: "RPK-74",
        level: 5,
        mass: 5.24,
        muzzle_velocity: 960.0,
        deviation: 0.012,
        fire_rate: 600.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_5_45X39,
        ammo_capacity: 45,
        reloading_time: Self::RELOADING_TIME_RIFLE_HEAVY,
        has_bolt: true,
        grip: WeaponGrip::TwoHandsWithButt,
        image_offset: 9.0,
    };

    pub const SAIGA_12K: Self = Self {
        name: "Saiga-12K",
        level: 6,
        mass: 3.3,
        muzzle_velocity: 410.0,
        deviation: 0.035,
        fire_rate: Self::FIRE_RATE_SHOTGUN,
        is_automatic: false,
        projectile: &ProjectileConfig::_12X76,
        ammo_capacity: 8,
        reloading_time: Self::RELOADING_TIME_RIFLE,
        has_bolt: true,
        grip: WeaponGrip::TwoHandsWithButt,
        image_offset: 9.0,
    };

    pub const PKM: Self = Self {
        name: "PKM",
        level: 7,
        mass: 7.5,
        muzzle_velocity: 825.0,
        deviation: 0.011,
        fire_rate: 650.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_7_62X54,
        ammo_capacity: 100,
        reloading_time: Self::RELOADING_TIME_MACHINE_GUN,
        has_bolt: true,
        grip: WeaponGrip::TwoHandsWithButt,
        image_offset: 10.0,
    };

    pub const PKP_PECHENEG: Self = Self {
        name: "PKP Pecheneg",
        level: 7,
        mass: 8.2,
        muzzle_velocity: 825.0,
        deviation: 0.01,
        fire_rate: 650.0,
        is_automatic: true,
        projectile: &ProjectileConfig::_7_62X54,
        ammo_capacity: 100,
        reloading_time: Self::RELOADING_TIME_MACHINE_GUN,
        has_bolt: true,
        grip: WeaponGrip::TwoHandsWithButt,
        image_offset: 10.0,
    };

    pub fn generate_velocity(&self, rng: &mut Pcg32) -> f32 {
        let deviation = rng.gen_normal(self.muzzle_velocity * Self::VELOCITY_DEVIATION);
        return self.muzzle_velocity + deviation;
    }

    pub fn generate_deviation(&self, rng: &mut Pcg32) -> f32 {
        return rng.gen_normal(self.deviation);
    }

    pub fn get_mass_with_full_ammo(&self) -> f32 {
        return self.mass + self.projectile.mass * f32::from(self.ammo_capacity);
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
            reloading: None,
            ammo: config.ammo_capacity,
            next_time: Duration::from_secs(0),
        };
    }

    pub fn try_fire(&mut self, time: Duration) -> bool {
        if self.is_ready(time) && self.has_ammo() {
            self.ammo = self.ammo.saturating_sub(1);
            self.next_time = time + Duration::from_secs_f32(60.0 / self.config.fire_rate);
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

pub enum WeaponGrip {
    OneHand,
    TwoHands,
    TwoHandsWithButt,
}

impl WeaponGrip {
    pub fn recoil_factor(&self) -> f32 {
        return match self {
            Self::OneHand => 0.5,
            Self::TwoHands => 0.75,
            Self::TwoHandsWithButt => 1.0,
        };
    }

    pub fn actor_image_suffix(&self) -> u8 {
        return match self {
            Self::OneHand => 1,
            Self::TwoHands => 2,
            Self::TwoHandsWithButt => 2,
        };
    }
}
