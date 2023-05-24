use crate::component::Actor;
use crate::model::ActorAction;
use bevy::ecs::system::Command;
use bevy::prelude::Entity;
use bevy::prelude::World;
use bevy::time::Time;

pub struct ActorMeleeReset(pub Entity);

impl Command for ActorMeleeReset {
    fn write(self, world: &mut World) {
        let time = world.resource::<Time>().elapsed();

        if let Some(mut actor) = world.get_mut::<Actor>(self.0) {
            actor.actions.remove(ActorAction::Attack);
            actor.melee_next = time + actor.config.melee_interval;
        }
    }
}
