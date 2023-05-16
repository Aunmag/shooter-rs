use crate::command::ActorSet;
use crate::component::Actor;
use crate::component::ActorConfig;
use crate::model::TransformLiteU8;
use crate::resource::Config;
use crate::resource::Message;
use crate::resource::NetResource;
use bevy::ecs::system::Command;
use bevy::prelude::Entity;
use bevy::prelude::Transform;
use bevy::prelude::World;
use std::net::SocketAddr;

pub struct ClientJoin(pub SocketAddr);

impl Command for ClientJoin {
    fn write(self, world: &mut World) {
        {
            let mut messages = Vec::with_capacity(64);
            let sync_interval = world.resource::<Config>().net.server.sync_interval;

            messages.push(Message::JoinAccept {
                id: 0,
                sync_interval,
            });

            for (entity, actor, transform) in
                world.query::<(Entity, &Actor, &Transform)>().iter(world)
            {
                messages.push(Message::ActorSpawn {
                    id: 0,
                    entity_index: entity.index(),
                    actor_type: actor.config.actor_type,
                    transform: transform.into(),
                });
            }

            let mut net = world.resource_mut::<NetResource>();
            for message in messages {
                net.send_to(&self.0, message);
            }
        }

        let entity = world.spawn_empty().id();

        ActorSet {
            entity,
            config: ActorConfig::HUMAN,
            transform: TransformLiteU8::default(),
            is_ghost: false,
        }
        .write(world);

        let mut net = world.resource_mut::<NetResource>();
        net.send_to(
            &self.0,
            Message::ActorGrant {
                id: 0,
                entity_index: entity.index(),
            },
        );
        net.attach_entity(&self.0, entity);
    }
}
