use crate::util::ext::RngExt;
use rand::Rng;
use std::{f32::consts::FRAC_PI_4, time::Duration};

pub struct BotConfig {
    pub is_silly: bool,
    pub is_agile: bool,
    pub reaction: Duration,
    pub spread: f32,
    pub sprint_distance: f32,
    pub shoot_distance_min: f32,
    pub shoot_distance_max: f32,
    pub angular_deviation: f32,
    pub shoot_prepare_duration: Duration,
    pub shoot_burst_duration: Duration,
    pub shoot_interval: Duration,
}

impl BotConfig {
    pub const IDLE_ROTATION: f32 = FRAC_PI_4;
    pub const IDLE_MOVEMENT_CHANCE: f64 = 0.1;
    pub const REPEAT_SHOOT_CHANCE: f64 = 0.6;
    pub const DODGE_ANGLE: f32 = FRAC_PI_4;

    pub const HUMAN: &'static Self = &Self {
        is_silly: false,
        is_agile: false,
        reaction: Duration::from_millis(250),
        spread: 0.0, // TODO: impl
        sprint_distance: 12.0,
        shoot_distance_min: 6.0,
        shoot_distance_max: 20.0,
        angular_deviation: 0.05,
        shoot_prepare_duration: Duration::from_millis(800),
        shoot_burst_duration: Duration::from_millis(400),
        shoot_interval: Duration::from_millis(300),
    };

    pub const ZOMBIE: &'static Self = &Self {
        is_silly: true,
        is_agile: false,
        reaction: Duration::from_millis(500),
        spread: 0.6, // TODO: 1.5 () is too high
        sprint_distance: Self::HUMAN.sprint_distance,
        shoot_distance_min: 3.0,
        shoot_distance_max: 10.0,
        angular_deviation: 0.16,
        shoot_prepare_duration: Duration::from_millis(1500),
        shoot_burst_duration: Duration::from_millis(400),
        shoot_interval: Duration::from_millis(800),
    };

    pub const ZOMBIE_AGILE: &'static Self = &Self {
        is_silly: true,
        is_agile: true,
        reaction: Self::HUMAN.reaction,
        spread: 0.1, // TODO: impl
        ..*Self::ZOMBIE
    };

    pub fn clone_with<R: Rng>(&self, skill: f32, r: &mut R) -> Self {
        return Self {
            is_silly: self.is_silly,
            is_agile: self.is_agile,
            reaction: r.fuzz_duration(self.reaction).div_f32(skill),
            // spread: r.gen_range_safely(0.5, 6.0), // TODO: use constants
            spread: 1.0, // TODO: use constants
            sprint_distance: r.fuzz(self.sprint_distance),
            shoot_distance_min: r.fuzz(self.shoot_distance_min),
            shoot_distance_max: r.fuzz(self.shoot_distance_max) * skill,
            angular_deviation: self.angular_deviation / skill,
            shoot_prepare_duration: self.shoot_prepare_duration.div_f32(skill),
            shoot_burst_duration: self.shoot_burst_duration,
            shoot_interval: self.shoot_interval.div_f32(skill),
        };
    }
}
