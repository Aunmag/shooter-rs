use crate::util::ext::RngExt;
use rand::Rng;
use std::f32::consts::FRAC_PI_4;

pub struct BotConfig {
    pub spread: f32,
    pub spread_force: f32,
    pub sprint_distance: f32,
}

impl BotConfig {
    pub const IDLE_ROTATION: f32 = FRAC_PI_4;
    pub const IDLE_MOVEMENT_CHANCE: f64 = 0.1;

    pub const HUMAN: &'static Self = &Self {
        spread: 0.8,
        spread_force: 0.2,
        sprint_distance: 10.0,
    };

    pub const ZOMBIE: &'static Self = &Self {
        spread: 3.0,
        spread_force: 0.4,
        sprint_distance: 8.0,
    };

    pub fn clone_with<R: Rng>(&self, r: &mut R) -> Self {
        return Self {
            spread: r.fuzz(self.spread),
            spread_force: f32::min(r.fuzz(self.spread_force), 1.0),
            sprint_distance: r.fuzz(self.sprint_distance),
        };
    }
}
