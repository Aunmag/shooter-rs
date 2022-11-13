use crate::component::Player;
use crate::resource::EntityConverter;
use crate::resource::Message;
use crate::resource::NetResource;
use crate::util::ext::WorldExt;
use bevy::ecs::system::Command;
use bevy::prelude::Entity;
use bevy::prelude::World;

pub struct EntityDelete(pub Entity);

impl Command for EntityDelete {
    fn write(self, world: &mut World) {
        if world.is_server() {
            world
                .resource_mut::<NetResource>()
                .send_to_all(Message::EntityDelete {
                    id: 0,
                    entity_id: self.0.id(),
                });
        }

        if let Some(ghost) = world.get::<Player>(self.0).and_then(|p| p.ghost) {
            world.despawn(ghost);
        }

        world.resource_mut::<EntityConverter>().remove(self.0);
        world.despawn(self.0);
    }
}
