use crate::{data::BotConfig, util::Timer};
use bevy::{ecs::component::Component, prelude::Entity};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::f32::consts::TAU;

#[derive(Component)]
pub struct Bot {
    pub config: BotConfig,
    pub enemy: Option<Entity>,
    pub teammates: Vec<Entity>,
    pub update_timer: Timer,
    pub voice_timer: Timer,
    pub idle_direction: f32,
    pub idle_movement: bool,
    pub rng: Pcg32,
}

impl Bot {
    pub fn new(config: &BotConfig, seed: u64) -> Self {
        let mut rng = Pcg32::seed_from_u64(seed);

        return Self {
            config: config.clone_with(&mut rng),
            enemy: None,
            teammates: Vec::new(),
            update_timer: Timer::default(),
            voice_timer: Timer::default(),
            idle_direction: rng.gen_range(0.0..TAU),
            idle_movement: false,
            rng,
        };
    }

    pub fn update_idle(&mut self) {
        self.idle_movement = self.rng.gen_bool(BotConfig::IDLE_MOVEMENT_CHANCE);
        self.idle_direction += self
            .rng
            .gen_range(-BotConfig::IDLE_ROTATION..BotConfig::IDLE_ROTATION);
    }
}
