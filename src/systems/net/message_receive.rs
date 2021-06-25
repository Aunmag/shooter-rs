use crate::components::ActorActions;
use crate::resources::EntityConverter;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use crate::resources::Message;
use crate::resources::NetConnection;
use crate::resources::NetResource;
use crate::resources::PositionUpdateResource;
use crate::resources::MESSAGE_SIZE_MAX;
use amethyst::ecs::Entities;
use amethyst::ecs::Entity;
use amethyst::ecs::System;
use amethyst::ecs::Write;
use std::io::ErrorKind;
use std::net::SocketAddr;

pub struct MessageReceiveSystem;

impl MessageReceiveSystem {
    #[allow(clippy::too_many_arguments)] // TODO: Simplify later
    fn on_message(
        address: &SocketAddr,
        message: &Message,
        entity: Option<Entity>,
        entities: &Entities,
        converter: &mut EntityConverter,
        tasks: &mut GameTaskResource,
        position_updates: &mut PositionUpdateResource,
        is_server: bool,
    ) {
        if is_server {
            Self::on_message_as_server(address, message, entity, tasks);
        } else {
            Self::on_message_as_client(message, entities, converter, tasks, position_updates);
        }
    }

    fn on_message_as_server(
        address: &SocketAddr,
        message: &Message,
        entity: Option<Entity>,
        tasks: &mut GameTaskResource,
    ) {
        match *message {
            Message::Join { .. } => {
                tasks.push(GameTask::ClientJoin(*address));
            }
            Message::ClientInput {
                actions, direction, ..
            } => {
                if let Some(entity) = entity {
                    tasks.push(GameTask::ActorAction {
                        entity,
                        actions: ActorActions::from_bits_truncate(actions),
                        direction,
                    });
                }
            }
            Message::ClientInputDirection { direction, .. } => {
                if let Some(entity) = entity {
                    tasks.push(GameTask::ActorTurn { entity, direction });
                }
            }
            _ => {}
        }
    }

    fn on_message_as_client(
        message: &Message,
        entities: &Entities,
        converter: &mut EntityConverter,
        tasks: &mut GameTaskResource,
        position_updates: &mut PositionUpdateResource,
    ) {
        match *message {
            Message::JoinAccept { .. } => {
                tasks.push(GameTask::Start);
            }
            Message::ActorSpawn {
                entity_id,
                actor_type,
                position,
                ..
            } => {
                tasks.push(GameTask::ActorSpawn {
                    entity: converter.to_internal(entities, entity_id),
                    actor_type: actor_type.into(),
                    position,
                });
            }
            Message::ActorGrant { entity_id, .. } => {
                tasks.push(GameTask::ActorGrant {
                    entity: converter.to_internal(entities, entity_id),
                });
            }
            Message::PositionUpdate {
                entity_id,
                position,
            } => {
                position_updates.insert(converter.to_internal(entities, entity_id).id(), position);
            }
            Message::ProjectileSpawn {
                position,
                velocity,
                acceleration_factor,
                shooter_id,
                ..
            } => {
                tasks.push(GameTask::ProjectileSpawn {
                    position,
                    velocity,
                    acceleration_factor,
                    shooter: shooter_id.map(|id| converter.to_internal(entities, id)),
                });
            }
            Message::EntityDelete { entity_id, .. } => {
                tasks.push(GameTask::EntityDelete(
                    converter.to_internal(entities, entity_id),
                ));
            }
            _ => {}
        }
    }
}

impl<'a> System<'a> for MessageReceiveSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, EntityConverter>,
        Write<'a, GameTaskResource>,
        Write<'a, PositionUpdateResource>,
        Option<Write<'a, NetResource>>,
    );

    fn run(
        &mut self,
        (entities, mut converter, mut tasks, mut position_updates, net): Self::SystemData,
    ) {
        let mut net = match net {
            Some(net) => net,
            None => return,
        };

        let is_server = net.is_server();

        let mut responses = Vec::new(); // TODO: Find a way send responses without vector allocations

        loop {
            let mut buffer = [0; MESSAGE_SIZE_MAX];

            match net.socket.recv_from(&mut buffer) {
                Ok((message_length, address)) => {
                    if !net.connections.contains_key(&address) {
                        log::info!("{} connected", address);
                    }

                    let message = buffer
                        .get(..message_length)
                        .ok_or_else(|| "Wrong message length".to_string())
                        .and_then(|m| Message::decode(m).map_err(|e| format!("{}", e)));

                    match message {
                        Ok(message) => {
                            let connection = net
                                .connections
                                .entry(address)
                                .or_insert_with(NetConnection::new);

                            if let Message::Response { message_id } = message {
                                connection.acknowledge_message(message_id);
                            } else {
                                if let Some(message_id) = message.get_id() {
                                    responses.push((address, Message::Response { message_id }));
                                }

                                if let Some(message) = connection.filter_message(message) {
                                    let entity = connection.attached_entity;
                                    let next_messages = connection.take_next_held_messages();

                                    Self::on_message(
                                        &address,
                                        &message,
                                        entity,
                                        &entities,
                                        &mut converter,
                                        &mut tasks,
                                        &mut position_updates,
                                        is_server,
                                    );

                                    for message in &next_messages {
                                        Self::on_message(
                                            &address,
                                            message,
                                            entity,
                                            &entities,
                                            &mut converter,
                                            &mut tasks,
                                            &mut position_updates,
                                            is_server,
                                        );
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            log::warn!("A corrupted message received from {}: {}", address, error);
                        }
                    }
                }
                Err(error) => {
                    if error.kind() == ErrorKind::WouldBlock {
                        break;
                    } else {
                        log::error!("Failed to receive new messages. {}", error);
                    }
                }
            }
        }

        for (address, message) in responses {
            net.send_to(&address, message);
        }
    }
}
