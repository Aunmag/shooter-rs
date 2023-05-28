use crate::{component::Actor, model::ActorAction};
use bevy::{
    ecs::system::Command,
    prelude::{Entity, World},
    time::Time,
};

pub struct ActorMeleeReset(pub Entity);

impl Command for ActorMeleeReset {
    fn write(self, world: &mut World) {
        let time = world.resource::<Time>().elapsed();

        if let Some(mut actor) = world.get_mut::<Actor>(self.0) {
            actor.actions.remove(ActorAction::Attack);
            actor.melee_next = time + actor.config.melee_interval.div_f32(actor.skill);
        }
    }
}
