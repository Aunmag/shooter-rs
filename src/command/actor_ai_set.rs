use crate::component::Ai;
use bevy::ecs::system::Command;
use bevy::prelude::Entity;
use bevy::prelude::World;

pub struct ActorAiSet(pub Entity);

impl Command for ActorAiSet {
    fn write(self, world: &mut World) {
        world.entity_mut(self.0).insert(Ai);
    }
}
