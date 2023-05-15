use crate::command::ActorActionsSet;
use crate::command::ActorDirectionSet;
use crate::command::ActorPlayerSet;
use crate::command::ActorSet;
use crate::command::ClientJoin;
use crate::command::EntityDelete;
use crate::command::ProjectileSpawn;
use crate::command::Start;
use crate::resource::EntityConverter;
use crate::resource::Message;
use crate::resource::NetConnection;
use crate::resource::NetResource;
use crate::resource::TransformUpdateResource;
use crate::resource::MESSAGE_SIZE_MAX;
use bevy::ecs::entity::Entities;
use bevy::ecs::entity::Entity;
use bevy::prelude::Commands;
use bevy::prelude::ResMut;
use std::io::ErrorKind;
use std::net::SocketAddr;

pub fn message_receive(
    entities: &Entities,
    mut commands: Commands,
    mut entity_converter: ResMut<EntityConverter>,
    mut transform_updates: ResMut<TransformUpdateResource>, // TODO: initialize for client only
    mut net: ResMut<NetResource>,
) {
    let is_server = net.is_server();

    let mut responses = Vec::new(); // TODO: find a way send responses without vector allocations

    loop {
        let mut buffer = [0; MESSAGE_SIZE_MAX];

        let (message_length, address) = match net.socket.recv_from(&mut buffer) {
            Ok((message_length, address)) => (message_length, address),
            Err(error) => {
                if error.kind() == ErrorKind::WouldBlock {
                    break;
                } else {
                    log::warn!("Failed to receive a message: {:?}", error);
                    continue;
                }
            }
        };

        let message = match Message::decode(&buffer[..message_length]) {
            Ok(message) => message,
            Err(error) => {
                log::warn!("A corrupted message received from {}: {:?}", address, error);
                continue;
            }
        };

        let connection = net.connections.entry(address).or_insert_with(|| {
            log::info!("{} connected", address);
            return NetConnection::new();
        });

        if let Message::Response { message_id } = message {
            connection.acknowledge_message(message_id);
        } else {
            if let Some(message_id) = message.get_id() {
                responses.push((address, Message::Response { message_id }));
            }

            if let Some(message) = connection.filter_message(message) {
                let entity = connection.attached_entity;
                let next_messages = connection.take_next_held_messages(); // TODO: optimize

                on_message(
                    &address,
                    &message,
                    entity,
                    entities,
                    &mut entity_converter,
                    &mut commands,
                    &mut transform_updates,
                    is_server,
                );

                for message in &next_messages {
                    on_message(
                        &address,
                        message,
                        entity,
                        entities,
                        &mut entity_converter,
                        &mut commands,
                        &mut transform_updates,
                        is_server,
                    );
                }
            }
        }
    }

    for (address, message) in responses {
        net.send_unreliably_to(&address, &message);
    }
}

fn on_message(
    address: &SocketAddr,
    message: &Message,
    entity: Option<Entity>,
    entities: &Entities,
    converter: &mut EntityConverter,
    commands: &mut Commands,
    transform_updates: &mut TransformUpdateResource,
    is_server: bool,
) {
    if is_server {
        on_message_as_server(address, message, entity, commands);
    } else {
        on_message_as_client(message, entities, converter, commands, transform_updates);
    }
}

fn on_message_as_server(
    address: &SocketAddr,
    message: &Message,
    entity: Option<Entity>,
    commands: &mut Commands,
) {
    match *message {
        Message::Join { .. } => {
            commands.add(ClientJoin(*address));
        }
        Message::ClientInput {
            actions, direction, ..
        } => {
            if let Some(entity) = entity {
                commands.add(ActorActionsSet {
                    entity,
                    actions,
                    direction,
                });
            }
        }
        Message::ClientInputDirection { direction, .. } => {
            if let Some(entity) = entity {
                commands.add(ActorDirectionSet { entity, direction });
            }
        }
        _ => {}
    }
}

fn on_message_as_client(
    message: &Message,
    entities: &Entities,
    converter: &mut EntityConverter,
    commands: &mut Commands,
    transform_updates: &mut TransformUpdateResource,
) {
    match *message {
        Message::JoinAccept { .. } => {
            commands.add(Start);
        }
        Message::ActorSpawn {
            entity_index,
            actor_type,
            transform,
            ..
        } => {
            commands.add(ActorSet {
                entity: converter.to_internal(entities, entity_index),
                config: actor_type.into(),
                transform,
                is_ghost: false,
            });
        }
        Message::ActorGrant { entity_index, .. } => {
            commands.add(ActorPlayerSet(
                converter.to_internal(entities, entity_index),
            ));
        }
        Message::TransformUpdate {
            entity_index,
            transform,
        } => {
            transform_updates.push((
                converter.to_internal(entities, entity_index).index(),
                transform,
            ));
        }
        Message::ProjectileSpawn {
            transform,
            velocity,
            acceleration_factor,
            shooter_id,
            ..
        } => {
            commands.add(ProjectileSpawn {
                transform,
                velocity,
                acceleration_factor,
                shooter: shooter_id.map(|id| converter.to_internal(entities, id)),
            });
        }
        Message::EntityDelete { entity_index, .. } => {
            commands.add(EntityDelete(converter.to_internal(entities, entity_index)));
        }
        _ => {}
    }
}
