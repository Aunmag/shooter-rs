use crate::{
    component::{Actor, Bot, Player},
    model::ActorActions,
};
use bevy::{
    ecs::system::Command,
    prelude::{Entity, World},
};

pub struct ActorRelease(pub Entity);

impl Command for ActorRelease {
    fn write(self, world: &mut World) {
        // TODO: find a way to stop all sounds
        if let Some(actor) = world.get_mut::<Actor>(self.0).as_mut() {
            actor.actions = ActorActions::EMPTY;
        }

        let mut entity_mut = world.entity_mut(self.0);
        entity_mut.remove::<Bot>();
        entity_mut.remove::<Player>();
    }
}
