use bevy::{ecs::component::Component, prelude::Entity};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::{ops::Range, time::Duration};

const SPRINT_DISTANCE: Range<f32> = 5.0..12.0;
const SPREAD: Range<f32> = 0.8..5.0;
/// Angle to turn while spreading out
/// 1.0 = 90 degrees
/// 0.5 = 45 degrees
const SPREAD_ANGULAR_FACTOR: Range<f32> = 0.2..0.5;
const STAMINA_MIN: Range<f32> = 0.15..0.4;

#[derive(Component)]
pub struct Bot {
    pub spread: f32,
    pub spread_angular_factor: f32,
    pub sprint_distance: f32,
    pub stamina_min: f32,
    pub enemy: Option<Entity>,
    pub teammates: Vec<Entity>,
    pub next_update: Duration,
    pub next_sound: Duration,
}

impl Bot {
    pub fn new(seed: u64) -> Self {
        let mut rng = Pcg32::seed_from_u64(seed);

        return Self {
            spread: rng.gen_range(SPREAD),
            spread_angular_factor: rng.gen_range(SPREAD_ANGULAR_FACTOR),
            sprint_distance: rng.gen_range(SPRINT_DISTANCE),
            stamina_min: rng.gen_range(STAMINA_MIN),
            enemy: None,
            teammates: Vec::new(),
            next_update: Duration::ZERO,
            next_sound: Duration::ZERO,
        };
    }
}
