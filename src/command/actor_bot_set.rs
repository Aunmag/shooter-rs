use crate::component::Bot;
use bevy::{
    ecs::system::Command,
    prelude::{Entity, World},
};

pub struct ActorBotSet(pub Entity);

impl Command for ActorBotSet {
    fn write(self, world: &mut World) {
        world.entity_mut(self.0).insert(Bot::default());
    }
}
