use bevy::ecs::system::Resource;
use derive_more::{Deref, DerefMut};
use rand::SeedableRng;
use rand_pcg::Pcg32;

#[derive(Resource, Deref, DerefMut)]
pub struct Rng(Pcg32);

impl Default for Rng {
    fn default() -> Self {
        return Self(Pcg32::seed_from_u64(10_005));
    }
}
