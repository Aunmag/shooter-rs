use crate::component::ProjectileConfig;
use bevy::ecs::component::Component;
use std::time::Duration;

#[derive(Component)]
pub struct Weapon {
    pub config: &'static WeaponConfig,
    next_shoot_time: Duration,
}

#[derive(Clone)]
pub struct WeaponConfig {
    pub name: &'static str,
    pub muzzle_velocity: f32,
    pub fire_rate: f32,
    pub projectile: &'static ProjectileConfig,
}

#[allow(dead_code)] // TODO: remove
impl WeaponConfig {
    const SEMI_AUTO_FIRE_RATE: f32 = 400.0;

    pub const LASER_GUN: Self = Self {
        name: "Laser Gun",
        muzzle_velocity: 5000.0,
        fire_rate: 1200.0,
        // radians_deflection: 0,
        // recoil: 0f,
        // is_automatic: true,
        // magazine_type: new MagazineType(ProjectileType.laser, true, 0, 0),
        // grip_offset: GRIP_OFFSET_EXTENDED,
        projectile: &ProjectileConfig::LASER,
    };

    pub const PM: Self = Self {
        name: "PM",
        muzzle_velocity: 315.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        // radians_deflection: 0.05,
        // recoil: 6_500,
        // is_automatic: false,
        // magazine_type: new MagazineType(ProjectileType._9x18mm_pm, true, 8, 2f),
        // grip_offset: GRIP_OFFSET_SHORT,
        projectile: &ProjectileConfig::_9X18,
    };

    pub const TT: Self = Self {
        name: "TT",
        muzzle_velocity: 430.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        // radians_deflection: 0.05,
        // recoil: 7_500,
        // is_automatic: false,
        // magazine_type: new MagazineType(ProjectileType._7_62x25mm_tt, true, 8, 2f),
        // grip_offset: GRIP_OFFSET_SHORT,
        projectile: &ProjectileConfig::_7_62X25,
    };

    pub const MP_43_SAWED_OFF: Self = Self {
        name: "MP-43 sawed-off",
        muzzle_velocity: 260.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        // radians_deflection: 0.08,
        // recoil: 45_000,
        // is_automatic: false,
        // magazine_type: new MagazineType(ProjectileType._12_76_magnum, false, 2, 0.5f),
        // grip_offset: GRIP_OFFSET_STEP * 4,
        projectile: &ProjectileConfig::_12X76,
    };

    pub const MP_27: Self = Self {
        name: "MP-27",
        muzzle_velocity: 410.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        // radians_deflection: 0.06,
        // recoil: 38_000,
        // is_automatic: false,
        // magazine_type: new MagazineType(ProjectileType._12_76_magnum, false, 2, 0.5f),
        // grip_offset: GRIP_OFFSET_EXTENDED,
        projectile: &ProjectileConfig::_12X76,
    };

    pub const PP_91_KEDR: Self = Self {
        name: "PP-91 Kedr",
        muzzle_velocity: 310.0,
        fire_rate: 900.0,
        // radians_deflection: 0.04,
        // recoil: 7_000,
        // is_automatic: true,
        // magazine_type: new MagazineType(ProjectileType._9x18mm_pm, true, 20, 1.8f),
        // grip_offset: GRIP_OFFSET_COMMON,
        projectile: &ProjectileConfig::_9X18,
    };

    pub const PP_19_BIZON: Self = Self {
        name: "PP-19 Bizon",
        muzzle_velocity: 330.0,
        fire_rate: 680.0,
        // radians_deflection: 0.03,
        // recoil: 7_500,
        // is_automatic: true,
        // magazine_type: new MagazineType(ProjectileType._9x18mm_pm, true, 64, 1.5f),
        // grip_offset: GRIP_OFFSET_COMMON,
        projectile: &ProjectileConfig::_9X18,
    };

    pub const AKS_74U: Self = Self {
        name: "AKS-74U",
        muzzle_velocity: 735.0,
        fire_rate: 675.0,
        // radians_deflection: 0.03,
        // recoil: 12_000,
        // is_automatic: true,
        // magazine_type: new MagazineType(ProjectileType._5_45x39mm, true, 30, 2f),
        // grip_offset: GRIP_OFFSET_COMMON,
        projectile: &ProjectileConfig::_5_45X39,
    };

    pub const AK_74M: Self = Self {
        name: "AK-74M",
        muzzle_velocity: 910.0,
        fire_rate: 600.0,
        // radians_deflection: 0.028,
        // recoil: 14_000,
        // is_automatic: true,
        // magazine_type: new MagazineType(ProjectileType._5_45x39mm, true, 30, 2f),
        // grip_offset: GRIP_OFFSET_COMMON,
        projectile: &ProjectileConfig::_5_45X39,
    };

    pub const RPK_74: Self = Self {
        name: "RPK-74",
        muzzle_velocity: 960.0,
        fire_rate: 600.0,
        // radians_deflection: 0.025,
        // recoil: 19_000,
        // is_automatic: true,
        // magazine_type: new MagazineType(ProjectileType._5_45x39mm, true, 45, 2f),
        // grip_offset: GRIP_OFFSET_COMMON,
        projectile: &ProjectileConfig::_5_45X39,
    };

    pub const SAIGA_12K: Self = Self {
        name: "Saiga-12K",
        muzzle_velocity: 410.0,
        fire_rate: Self::SEMI_AUTO_FIRE_RATE,
        // radians_deflection: 0.07,
        // recoil: 32_000,
        // is_automatic: false,
        // magazine_type: new MagazineType(ProjectileType._12_76_magnum, true, 8, 2f),
        // grip_offset: GRIP_OFFSET_COMMON,
        projectile: &ProjectileConfig::_12X76,
    };

    pub const PKM: Self = Self {
        name: "PKM",
        muzzle_velocity: 825.0,
        fire_rate: 650.0,
        // radians_deflection: 0.021,
        // recoil: 22_000,
        // is_automatic: true,
        // magazine_type: new MagazineType(ProjectileType._7_62x54mmR, true, 100, 8f),
        // grip_offset: GRIP_OFFSET_EXTENDED,
        projectile: &ProjectileConfig::_7_62X54,
    };

    pub const PKP_PECHENEG: Self = Self {
        name: "PKP Pecheneg",
        muzzle_velocity: 825.0,
        fire_rate: 650.0,
        // radians_deflection: 0.02,
        // recoil: 22_000,
        // is_automatic: true,
        // magazine_type: new MagazineType(ProjectileType._7_62x54mmR, true, 100, 8f),
        // grip_offset: GRIP_OFFSET_EXTENDED,
        projectile: &ProjectileConfig::_7_62X54,
    };
}

impl Weapon {
    pub const fn new(config: &'static WeaponConfig) -> Self {
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
