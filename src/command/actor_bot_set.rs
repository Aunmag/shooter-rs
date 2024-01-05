use crate::component::{Actor, Bot};
use bevy::{
    ecs::system::Command,
    prelude::{Entity, World},
};

pub struct ActorBotSet(pub Entity);

impl Command for ActorBotSet {
    fn apply(self, world: &mut World) {
        let entity_id = u64::from(self.0.index());

        if let Some(config) = world
            .get::<Actor>(self.0)
            .map(|a| a.config.kind.get_bot_config())
        {
            world.entity_mut(self.0).insert(Bot::new(config, entity_id));
        } else {
            log::warn!("Can't set bot. Entity has no actor component");
        }
    }
}
