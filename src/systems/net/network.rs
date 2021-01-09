use crate::components::ActorActions;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use crate::resources::Message;
use crate::resources::MessageReceiver;
use crate::resources::MessageResource;
use crate::resources::NetworkTask;
use crate::resources::NetworkTaskResource;
use crate::resources::MESSAGE_SIZE_MAX;
use crate::systems::net::connection::Connection;
use crate::systems::net::connection::ConnectionStatus;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::Write;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::net::UdpSocket;

#[derive(SystemDesc)]
pub struct NetworkSystem {
    socket: UdpSocket,
    is_server: bool,
    connections: HashMap<SocketAddr, Connection>,
}

impl NetworkSystem {
    pub fn new_as_server(port: u16) -> Result<Self, String> {
        return Self::new(&format!("0.0.0.0:{}", port), true);
    }

    pub fn new_as_client(server_address: SocketAddr) -> Result<Self, String> {
        let mut network = Self::new("0.0.0.0:0", false)?;
        network
            .connections
            .insert(server_address, Connection::new());

        network.send(&MessageReceiver::Every, Message::Greeting { id: 0 });
        return Ok(network);
    }

    fn new(address: &str, is_server: bool) -> Result<Self, String> {
        let socket = UdpSocket::bind(address).map_err(|e| format!("{}", e))?;
        socket.set_nonblocking(true).map_err(|e| format!("{}", e))?;

        return Ok(Self {
            socket,
            is_server,
            connections: HashMap::new(),
        });
    }

    fn send_outgoing_messages(&mut self, messages: &mut MessageResource) {
        for (receiver, message) in messages.drain(..) {
            self.send(&receiver, message);
        }
    }

    fn update_connections(&mut self) {
        let mut disconnected = Vec::new();

        for (address, connection) in self.connections.iter_mut() {
            connection.resend_unacknowledged_messages(&self.socket, &address);

            if let ConnectionStatus::Disconnected(ref reason) = *connection.get_status() {
                disconnected.push(*address);
                log::warn!("{} disconnected. {}", address, reason);
            }
        }

        for address in disconnected.iter() {
            self.connections.remove(address);
        }
    }

    fn process_incoming_tasks(&mut self, tasks: &mut NetworkTaskResource) {
        for task in tasks.drain(..) {
            match task {
                NetworkTask::AttachEntity {
                    address,
                    external_id,
                } => {
                    if let Some(connection) = self.connections.get_mut(&address) {
                        connection.attached_external_id.replace(external_id);
                    }
                }
            }
        }
    }

    fn read_incoming_messages(&mut self, tasks: &mut GameTaskResource) {
        loop {
            let mut buffer = [0; MESSAGE_SIZE_MAX];

            match self.socket.recv_from(&mut buffer) {
                Ok((message_length, address)) => {
                    if !self.connections.contains_key(&address) {
                        log::info!("{} connected", address);
                    }

                    let message = buffer
                        .get(..message_length)
                        .ok_or_else(|| "Wrong message length".to_string())
                        .and_then(|m| Message::decode(m).map_err(|e| format!("{}", e)));

                    match message {
                        Ok(message) => {
                            let connection = self
                                .connections
                                .entry(address)
                                .or_insert_with(Connection::new);

                            if let Message::Response { message_id } = message {
                                connection.acknowledge_message(message_id);
                            } else {
                                if let Some(message_id) = message.get_id() {
                                    connection.send(
                                        &self.socket,
                                        &address,
                                        &mut Message::Response { message_id },
                                    );
                                }

                                if let Some(message) = connection.filter_message(message) {
                                    let external_id = connection.attached_external_id;
                                    let next_messages = connection.take_next_held_messages();

                                    self.on_message(&address, &message, external_id, tasks);

                                    for message in next_messages.iter() {
                                        self.on_message(&address, &message, external_id, tasks);
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            self.connections.remove(&address);
                            log::warn!(
                                "{} disconnected. A corrupted message received. {}",
                                address,
                                error,
                            );
                            // TODO: Notify address
                        }
                    }
                }
                Err(error) => {
                    if error.kind() == ErrorKind::WouldBlock {
                        break;
                    } else {
                        log::error!("Failed to receive new messages. {}", error);
                        // TODO: Close connection
                    }
                }
            }
        }
    }

    fn on_message(
        &mut self,
        address: &SocketAddr,
        message: &Message,
        external_id: Option<u16>,
        tasks: &mut GameTaskResource,
    ) {
        if self.is_server {
            Self::on_message_as_server(&address, &message, external_id, tasks);
        } else {
            Self::on_message_as_client(&message, tasks);
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
                } else {
                    // TODO: Kick
                }
            }
            Message::ClientInputDirection { direction, .. } => {
                if let Some(external_id) = external_id {
                    tasks.push(GameTask::ActorTurn {
                        external_id,
                        direction,
                    });
                } else {
                    // TODO: Kick
                }
            }
            _ => {
                // TODO: Kick
            }
        }
    }

    fn on_message_as_client(message: &Message, tasks: &mut GameTaskResource) {
        match *message {
            Message::ActorSpawn {
                external_id,
                x,
                y,
                direction,
                ..
            } => {
                tasks.push(GameTask::ActorSpawn {
                    external_id,
                    x,
                    y,
                    direction,
                });
            }
            Message::ActorGrant { external_id, .. } => {
                tasks.push(GameTask::ActorGrant { external_id });
            }
            Message::TransformSync {
                external_id,
                x,
                y,
                direction,
                ..
            } => {
                tasks.push(GameTask::TransformSync {
                    external_id,
                    x,
                    y,
                    direction,
                });
            }
            Message::ProjectileSpawn {
                x,
                y,
                velocity_x,
                velocity_y,
                acceleration_factor,
                ..
            } => {
                tasks.push(GameTask::ProjectileSpawn {
                    x,
                    y,
                    velocity_x,
                    velocity_y,
                    acceleration_factor,
                });
            }
            _ => {}
        }
    }

    fn send(&mut self, receiver: &MessageReceiver, mut message: Message) {
        match *receiver {
            MessageReceiver::Only(ref address) => {
                if let Some(connection) = self.connections.get_mut(address) {
                    connection.send(&self.socket, address, &mut message);
                }
            }
            MessageReceiver::Every => {
                for (address, connection) in self.connections.iter_mut() {
                    connection.send(&self.socket, &address, &mut message);
                }
            }
        }
    }
}

impl<'a> System<'a> for NetworkSystem {
    type SystemData = (
        Write<'a, GameTaskResource>,
        Write<'a, MessageResource>,
        Write<'a, NetworkTaskResource>,
    );

    fn run(&mut self, (mut game_tasks, mut messages, mut network_tasks): Self::SystemData) {
        self.send_outgoing_messages(&mut messages);
        self.update_connections();
        self.process_incoming_tasks(&mut network_tasks);
        self.read_incoming_messages(&mut game_tasks);
    }
}
