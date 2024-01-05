use crate::util::ext::RngExt;
use rand::Rng;
use std::{f32::consts::FRAC_PI_4, time::Duration};

pub struct BotConfig {
    pub reaction: Duration,
    pub spread: f32,
    pub spread_force: f32,
    pub sprint_distance: f32,
}

impl BotConfig {
    pub const IDLE_ROTATION: f32 = FRAC_PI_4;
    pub const IDLE_MOVEMENT_CHANCE: f64 = 0.1;

    pub const HUMAN: &'static Self = &Self {
        reaction: Duration::from_millis(250),
        spread: 0.8,
        spread_force: 0.2,
        sprint_distance: 10.0,
    };

    pub const ZOMBIE: &'static Self = &Self {
        reaction: Duration::from_millis(500),
        spread: 3.0,
        spread_force: 0.4,
        sprint_distance: 8.0,
    };

    pub fn clone_with<R: Rng>(&self, skill: f32, r: &mut R) -> Self {
        return Self {
            reaction: r.fuzz_duration(self.reaction).div_f32(skill),
            spread: r.fuzz(self.spread),
            spread_force: f32::min(r.fuzz(self.spread_force), 1.0),
            sprint_distance: r.fuzz(self.sprint_distance),
        };
    }
}
