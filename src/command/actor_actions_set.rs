use crate::{component::Actor, model::ActorActions};
use bevy::{
    ecs::system::Command,
    math::Quat,
    prelude::{Entity, Transform, World},
};

pub struct ActorActionsSet {
    pub entity: Entity,
    pub actions: ActorActions,
    pub direction: f32,
}

impl Command for ActorActionsSet {
    fn write(self, world: &mut World) {
        if let Some(mut actor) = world.get_mut::<Actor>(self.entity) {
            actor.actions = self.actions;
        }

        if let Some(mut transform) = world.get_mut::<Transform>(self.entity) {
            transform.rotation = Quat::from_rotation_z(self.direction);
        }
    }
}
