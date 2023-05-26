use crate::component::Bot;
use bevy::{
    ecs::system::Command,
    prelude::{Entity, World},
};

pub struct ActorBotSet(pub Entity);

impl Command for ActorBotSet {
    fn write(self, world: &mut World) {
        let entity_id = u64::from(self.0.index());
        world.entity_mut(self.0).insert(Bot::new(entity_id));
    }
}
