use crate::util::Interpolation;
use bevy::{ecs::component::Component, prelude::Entity};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::{f32::consts::FRAC_PI_2, time::Duration};

const DIRECTION_DISTORTION: f32 = FRAC_PI_2 * 0.4;

#[derive(Component)]
pub struct Bot {
    pub target_actor: Option<Entity>,
    pub target_point: Option<Interpolation>,
    pub direction_distortion: f32,
    pub next_sound: Duration,
}

impl Bot {
    pub fn new(seed: u64) -> Self {
        let direction_distortion =
            Pcg32::seed_from_u64(seed).gen_range(-DIRECTION_DISTORTION..DIRECTION_DISTORTION);

        return Self {
            target_actor: None,
            target_point: None,
            direction_distortion,
            next_sound: Duration::ZERO,
        };
    }
}
