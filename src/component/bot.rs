use crate::{
    data::BotConfig,
    util::{ext::RngExt, Timer},
};
use bevy::{ecs::component::Component, prelude::Entity};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::{f32::consts::TAU, time::Duration};

#[derive(Component)]
pub struct Bot {
    pub config: BotConfig,
    pub enemy: Option<Entity>,
    pub teammates: Vec<Entity>,
    pub update_timer: Timer,
    pub voice_timer: Timer,
    pub idle_direction: f32,
    pub idle_movement: bool,
    pub was_burst_fire: bool,
    pub rng: Pcg32,
    shooting_state: BotShootingState,
    shooting_timer: Timer,
}

impl Bot {
    pub fn new(config: &BotConfig, skill: f32, seed: u64) -> Self {
        let mut rng = Pcg32::seed_from_u64(seed);

        return Self {
            config: config.clone_with(skill, &mut rng),
            enemy: None,
            teammates: Vec::new(),
            update_timer: Timer::default(),
            voice_timer: Timer::default(),
            idle_direction: rng.gen_range(0.0..TAU),
            idle_movement: false,
            was_burst_fire: false,
            shooting_state: BotShootingState::Prepare,
            shooting_timer: Timer::default(),
            rng,
        };
    }

    pub fn update_idle(&mut self) {
        self.idle_movement = self.rng.gen_bool(BotConfig::IDLE_MOVEMENT_CHANCE);
        self.idle_direction += self
            .rng
            .gen_range(-BotConfig::IDLE_ROTATION..BotConfig::IDLE_ROTATION);
    }

    pub fn get_shooting_state(
        &mut self,
        is_weapon_automatic: bool,
        time: Duration,
    ) -> BotShootingState {
        if self.shooting_timer.is_ready_and_enabled(time) {
            let next_state = match self.shooting_state {
                BotShootingState::Prepare => BotShootingState::Shoot,
                BotShootingState::Shoot => BotShootingState::Pause,
                BotShootingState::Pause => {
                    if self.rng.gen_bool(BotConfig::REPEAT_SHOOT_CHANCE) {
                        BotShootingState::Shoot
                    } else {
                        BotShootingState::Prepare
                    }
                }
            };

            self.set_shooting_state(next_state, is_weapon_automatic, time);
        }

        return self.shooting_state;
    }

    pub fn set_shooting_state(
        &mut self,
        state: BotShootingState,
        is_weapon_automatic: bool,
        time: Duration,
    ) {
        let duration = match state {
            BotShootingState::Prepare => self.config.shoot_prepare_duration,
            BotShootingState::Shoot => {
                if is_weapon_automatic {
                    self.config.shoot_burst_duration
                } else {
                    Duration::ZERO // longer time can result ActorAction::Attack changing multiple times
                }
            }
            BotShootingState::Pause => self.config.shoot_interval,
        };

        self.shooting_state = state;
        self.shooting_timer
            .set(time + self.rng.fuzz_duration(duration));
    }

    pub fn set_shooting_target(&mut self, has_target: bool, time: Duration) {
        let was_target = self.shooting_timer.is_enabled();

        match (was_target, has_target) {
            // target appeared
            (false, true) => {
                self.set_shooting_state(BotShootingState::Prepare, false, time);
            }
            // target disappeared
            (true, false) => {
                self.shooting_state = BotShootingState::Prepare;
                self.shooting_timer.disable();
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BotShootingState {
    Prepare,
    Shoot,
    Pause,
}
