use bevy::{
    ecs::system::Command,
    math::Quat,
    prelude::{Entity, Transform, World},
};

pub struct ActorDirectionSet {
    pub entity: Entity,
    pub direction: f32,
}

impl Command for ActorDirectionSet {
    fn write(self, world: &mut World) {
        if let Some(mut transform) = world.get_mut::<Transform>(self.entity) {
            transform.rotation = Quat::from_rotation_z(self.direction);
        }
    }
}
