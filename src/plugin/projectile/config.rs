use std::time::Duration;

#[derive(Clone)]
pub struct ProjectileConfig {
    pub fragments: u8,
    pub mass: f32,
    pub size: f32,
    pub physics: ProjectilePhysics,
    pub explosion: Option<ProjectileExplosion>,
}

impl ProjectileConfig {
    /// Will stop projectiles that barely move. Essential for bullets with constant deceleration so
    /// they drop at slow speed
    pub const VELOCITY_MIN: f32 = 5.0;

    /// Makes game more caricature by slowing projectiles down, but the physics still works as if
    /// they travel with real-world velocities
    pub const VELOCITY_VISUAL_FACTOR: f32 = 1.0 / 5.0;

    pub const ROCKET_ACCELERATION_FACTOR: f32 = 8.0;
    pub const ROCKET_ACCELERATION_TIME: Duration = Duration::from_millis(800);

    pub const _9X18: Self = Self {
        fragments: 1,
        mass: 0.0061,
        size: 0.7,
        physics: ProjectilePhysics::Bullet,
        explosion: None,
    };

    pub const _7_62X25: Self = Self {
        fragments: 1,
        mass: 0.0055,
        size: 0.7,
        physics: ProjectilePhysics::Bullet,
        explosion: None,
    };

    pub const _12X76: Self = Self {
        fragments: 12,
        mass: 0.048,
        size: 0.1,
        physics: ProjectilePhysics::Bullet,
        explosion: None,
    };

    pub const _5_45X39: Self = Self {
        fragments: 1,
        mass: 0.0034,
        size: 1.0,
        physics: ProjectilePhysics::Bullet,
        explosion: None,
    };

    pub const _7_62X54: Self = Self {
        fragments: 1,
        mass: 0.0096,
        size: 1.2,
        physics: ProjectilePhysics::Bullet,
        explosion: None,
    };

    pub const TBG_7V: Self = Self {
        fragments: 1,
        mass: 4.3,
        size: 5.0,
        physics: ProjectilePhysics::Rocket,
        explosion: Some(ProjectileExplosion {
            radius: 4.0,
            energy: 8.0,
        }),
    };

    pub const fn acceleration(&self) -> f32 {
        return -1.0 / self.fragment_mass() * 0.006 - 4.2;
    }

    pub const fn fragment_mass(&self) -> f32 {
        return self.mass / self.fragments as f32;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProjectilePhysics {
    Bullet,
    Rocket,
}

impl ProjectilePhysics {
    pub const fn distance_limit(&self) -> f32 {
        return match self {
            Self::Bullet => f32::INFINITY,
            Self::Rocket => 40.0,
        };
    }
}

#[derive(Clone)]
pub struct ProjectileExplosion {
    pub radius: f32,
    pub energy: f32,
}
