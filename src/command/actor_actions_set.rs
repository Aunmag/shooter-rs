use crate::component::Actor;
use crate::component::ActorActions;
use bevy::ecs::system::Command;
use bevy::math::Quat;
use bevy::prelude::Entity;
use bevy::prelude::Transform;
use bevy::prelude::World;

pub struct ActorActionsSet {
    pub entity: Entity,
    pub actions: ActorActions,
    pub direction: f32,
}

impl Command for ActorActionsSet {
    fn write(self, world: &mut World) {
        let mut entity_mut = world.entity_mut(self.entity);

        if let Some(mut actor) = entity_mut.get_mut::<Actor>() {
            actor.actions = self.actions;
        }

        if let Some(mut transform) = entity_mut.get_mut::<Transform>() {
            transform.rotation = Quat::from_rotation_z(self.direction);
        }
    }
}
