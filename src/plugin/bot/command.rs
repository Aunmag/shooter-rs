use super::Bot;
use crate::plugin::actor::Actor;
use bevy::{
    ecs::system::Command,
    prelude::{Entity, World},
};

pub struct ActorBotSet {
    pub entity: Entity,
}

impl Command for ActorBotSet {
    type Out = ();

    fn apply(self, world: &mut World) {
        let entity_id = self.entity.index_u32();

        if let Some(actor) = world.get::<Actor>(self.entity) {
            let config = actor.config.bot;
            let skill = actor.skill;

            world
                .entity_mut(self.entity)
                .insert(Bot::new(config, skill, entity_id));
        } else {
            log::warn!("Can't set bot. Entity has no actor component");
        }
    }
}
