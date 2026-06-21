#[derive(Clone)]
pub struct ProjectileConfig {
    pub fragments: u8,
    pub mass: f32,
    pub size: f32,
    pub is_rocket: bool,
}

impl ProjectileConfig {
    pub const _9X18: Self = Self {
        fragments: 1,
        mass: 0.0061,
        size: 0.7,
        is_rocket: false,
    };

    pub const _7_62X25: Self = Self {
        fragments: 1,
        mass: 0.0055,
        size: 0.7,
        is_rocket: false,
    };

    pub const _12X76: Self = Self {
        fragments: 12,
        mass: 0.048,
        size: 0.1,
        is_rocket: false,
    };

    pub const _5_45X39: Self = Self {
        fragments: 1,
        mass: 0.0034,
        size: 1.0,
        is_rocket: false,
    };

    pub const _7_62X54: Self = Self {
        fragments: 1,
        mass: 0.0096,
        size: 1.2,
        is_rocket: false,
    };

    pub const TBG_7V: Self = Self {
        fragments: 1,
        mass: 4.1,
        size: 5.0,
        is_rocket: true,
    };

    pub fn acceleration(&self) -> f32 {
        return -1.0 / self.fragment_mass() * 0.006 - 4.2;
    }

    pub fn fragment_mass(&self) -> f32 {
        return self.mass / f32::from(self.fragments);
    }
}
