use super::Bot;
use crate::component::Actor;
use bevy::{
    ecs::system::Command,
    prelude::{Entity, World},
};

pub struct ActorBotSet {
    pub entity: Entity,
}

impl Command for ActorBotSet {
    fn apply(self, world: &mut World) {
        let mut entity = world.entity_mut(self.entity);

        if let Some((config, skill)) = entity.get::<Actor>().map(|a| (a.config.bot, a.skill)) {
            let seed = u64::from(self.entity.index());
            entity.insert(Bot::new(config, skill, seed));
        } else {
            log::warn!("Can't set bot. Entity has no actor component");
        }
    }
}
