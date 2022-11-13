use bevy::ecs::system::Command;
use bevy::math::Quat;
use bevy::prelude::Entity;
use bevy::prelude::Transform;
use bevy::prelude::World;

pub struct ActorDirectionSet {
    pub entity: Entity,
    pub direction: f32,
}

impl Command for ActorDirectionSet {
    fn write(self, world: &mut World) {
        if let Some(mut transform) = world.entity_mut(self.entity).get_mut::<Transform>() {
            transform.rotation = Quat::from_rotation_z(self.direction);
        }
    }
}
