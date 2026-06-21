use crate::{map::Map, plugin::TerrainSpawn};
use bevy::ecs::world::World;

pub struct TestMap;

impl Map for TestMap {
    fn generate(&self, world: &mut World) {
        world.commands().add(TerrainSpawn {
            image: "terrain/test.png",
        });
    }
}
