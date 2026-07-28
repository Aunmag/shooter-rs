use super::Bot;
use crate::plugin::{actor::Actor, bot::voice::BotVoice, ActorKind};
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
            let kind = actor.config.kind;
            let config = actor.config.bot;
            let skill = actor.skill;

            let mut entity = world.entity_mut(self.entity);

            entity.insert(Bot::new(config, skill, entity_id));

            if kind != ActorKind::Human {
                entity.insert(BotVoice::default());
            }
        } else {
            log::warn!("Can't set bot. Entity has no actor component");
        }
    }
}
