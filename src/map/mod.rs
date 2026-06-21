mod forest;
mod test;

pub use self::{forest::*, test::*};
use bevy::ecs::world::World;

pub trait Map {
    fn generate(&self, world: &mut World);
}
