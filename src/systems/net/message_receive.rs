use crate::components::ActorActions;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use crate::resources::Message;
use crate::resources::NetConnection;
use crate::resources::NetResource;
use crate::resources::PositionUpdateResource;
use crate::resources::MESSAGE_SIZE_MAX;
use amethyst::ecs::System;
use amethyst::ecs::Write;
use std::io::ErrorKind;
use std::net::SocketAddr;

pub struct MessageReceiveSystem;

impl MessageReceiveSystem {
    fn on_message(
        address: &SocketAddr,
        message: &Message,
        external_id: Option<u16>,
        tasks: &mut GameTaskResource,
        position_updates: &mut PositionUpdateResource,
        is_server: bool,
    ) {
        if is_server {
            Self::on_message_as_server(&address, &message, external_id, tasks);
        } else {
            Self::on_message_as_client(&message, tasks, position_updates);
        }
    }

    fn on_message_as_server(
        address: &SocketAddr,
        message: &Message,
        external_id: Option<u16>,
        tasks: &mut GameTaskResource,
    ) {
        match *message {
            Message::Greeting { .. } => {
                tasks.push(GameTask::ClientGreet(*address));
            }
            Message::ClientInput {
                actions, direction, ..
            } => {
                if let Some(external_id) = external_id {
                    tasks.push(GameTask::ActorAction {
                        external_id,
                        actions: ActorActions::from_bits_truncate(actions),
                        direction,
                    });
                }
            }
            Message::ClientInputDirection { direction, .. } => {
                if let Some(external_id) = external_id {
                    tasks.push(GameTask::ActorTurn {
                        external_id,
                        direction,
                    });
                }
            }
            _ => {}
        }
    }

    fn on_message_as_client(
        message: &Message,
        tasks: &mut GameTaskResource,
        position_updates: &mut PositionUpdateResource,
    ) {
        match *message {
            Message::ActorSpawn {
                external_id,
                position,
                ..
            } => {
                tasks.push(GameTask::ActorSpawn {
                    external_id,
                    position,
                });
            }
            Message::ActorGrant { external_id, .. } => {
                tasks.push(GameTask::ActorGrant { external_id });
            }
            Message::PositionUpdate {
                external_id,
                position,
            } => {
                position_updates.insert(external_id, position);
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
                    shooter_id,
                });
            }
            _ => {}
        }
    }
}

impl<'a> System<'a> for MessageReceiveSystem {
    type SystemData = (
        Write<'a, GameTaskResource>,
        Write<'a, PositionUpdateResource>,
        Option<Write<'a, NetResource>>,
    );

    fn run(&mut self, (mut tasks, mut position_updates, net): Self::SystemData) {
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
                                    let external_id = connection.attached_external_id;
                                    let next_messages = connection.take_next_held_messages();

                                    Self::on_message(
                                        &address,
                                        &message,
                                        external_id,
                                        &mut tasks,
                                        &mut position_updates,
                                        is_server,
                                    );

                                    for message in next_messages.iter() {
                                        Self::on_message(
                                            &address,
                                            &message,
                                            external_id,
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
