use crate::{
    component::Player,
    resource::{EntityConverter, Message, NetResource},
    util::ext::WorldExt,
};
use bevy::{
    ecs::system::Command,
    prelude::{DespawnRecursiveExt, Entity, World},
};

pub struct EntityDelete(pub Entity);

impl Command for EntityDelete {
    fn write(self, world: &mut World) {
        if world.is_server() {
            world
                .resource_mut::<NetResource>()
                .send_to_all(Message::EntityDelete {
                    id: 0,
                    entity_index: self.0.index(),
                });
        }

        if let Some(ghost) = world.get::<Player>(self.0).and_then(|p| p.ghost) {
            world.entity_mut(ghost).despawn_recursive();
        }

        world.resource_mut::<EntityConverter>().remove(self.0);
        world.entity_mut(self.0).despawn_recursive();
    }
}
