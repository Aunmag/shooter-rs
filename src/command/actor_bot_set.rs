use crate::component::{Actor, Bot};
use bevy::{
    ecs::system::Command,
    prelude::{Entity, World},
};

pub struct ActorBotSet {
    pub entity: Entity,
    pub skill: f32,
}

impl Command for ActorBotSet {
    fn apply(self, world: &mut World) {
        let entity_id = u64::from(self.entity.index());

        if let Some(config) = world.get::<Actor>(self.entity).map(|a| a.config.bot) {
            world
                .entity_mut(self.entity)
                .insert(Bot::new(config, self.skill, entity_id));
        } else {
            log::warn!("Can't set bot. Entity has no actor component");
        }
    }
}
