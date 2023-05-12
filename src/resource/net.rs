use crate::resource::Message;
use crate::resource::NetConfig;
use bevy::ecs::system::Resource;
use bevy::prelude::Entity;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::time::Duration;
use std::time::Instant;

#[derive(Resource)]
pub struct NetResource {
    pub socket: UdpSocket,
    pub connections: HashMap<SocketAddr, NetConnection>,
    is_server: bool, // TODO: try to avoid
    message_resend_interval: Duration,
}

pub struct NetConnection {
    status: NetConnectionStatus,
    // TODO: maybe don't allow grow to large
    unacknowledged_messages: HashMap<u16, UnacknowledgedMessage>,
    // TODO: maybe don't allow grow to large
    held_messages: HashMap<u16, Message>,
    // TODO: handle ID restart
    next_incoming_message_id: u16,
    next_outgoing_message_id: u16,
    pub attached_entity: Option<Entity>,
}

pub enum NetConnectionStatus {
    Connected,
    Disconnected(String),
}

struct UnacknowledgedMessage {
    data: Vec<u8>,
    last_sent: Instant,
}

impl NetResource {
    pub fn new_as_server(config: &NetConfig) -> Result<Self, String> {
        return Self::new(&format!("0.0.0.0:{}", config.server.port), config, true);
    }

    pub fn new_as_client(config: &NetConfig) -> Result<Self, String> {
        let mut network = Self::new("0.0.0.0:0", config, false)?;

        network
            .connections
            .insert(config.client.join, NetConnection::new());

        network.send_to_all(Message::Join { id: 0 });

        return Ok(network);
    }

    fn new(address: &str, config: &NetConfig, is_server: bool) -> Result<Self, String> {
        let socket = UdpSocket::bind(address).map_err(|e| format!("{}", e))?;
        socket.set_nonblocking(true).map_err(|e| format!("{}", e))?;

        return Ok(Self {
            is_server,
            socket,
            connections: HashMap::new(),
            message_resend_interval: config.message_resend_interval,
        });
    }

    pub fn update_connections(&mut self) {
        let mut disconnected = Vec::new();

        for (address, connection) in &mut self.connections {
            connection.resend_unacknowledged_messages(
                &self.socket,
                address,
                self.message_resend_interval,
            );

            if let NetConnectionStatus::Disconnected(ref reason) = *connection.get_status() {
                disconnected.push(*address);
                log::warn!("{} disconnected. {}", address, reason);
            }
        }

        for address in &disconnected {
            self.connections.remove(address);
        }
    }

    pub fn send_to(&mut self, address: &SocketAddr, mut message: Message) {
        if let Some(connection) = self.connections.get_mut(address) {
            connection.send(&self.socket, address, &mut message);
        }
    }

    pub fn send_to_all(&mut self, mut message: Message) {
        for (address, connection) in &mut self.connections {
            connection.send(&self.socket, address, &mut message);
        }
    }

    /// A faster way to send a message by skipping acknowledgement and error handling.
    pub fn send_unreliably_to(&self, address: &SocketAddr, message: &Message) {
        if let Err(error) = self.socket.send_to(&message.encode(), address) {
            log::warn!("Failed to unreliable message to {}: {:?}", address, error);
        }
    }

    /// A faster way to send a message by skipping acknowledgement and error handling.
    pub fn send_unreliably_to_all(&self, message: &Message) {
        let encoded = message.encode();

        for address in self.connections.keys() {
            if let Err(error) = self.socket.send_to(&encoded, address) {
                log::warn!("Failed to unreliable message to {}: {:?}", address, error);
            }
        }
    }

    pub fn attach_entity(&mut self, address: &SocketAddr, entity: Entity) {
        if let Some(connection) = self.connections.get_mut(address) {
            connection.attached_entity.replace(entity);
        }
    }

    pub const fn is_server(&self) -> bool {
        return self.is_server;
    }
}

impl NetConnection {
    pub fn new() -> Self {
        return Self {
            status: NetConnectionStatus::Connected,
            unacknowledged_messages: HashMap::new(),
            held_messages: HashMap::new(),
            next_incoming_message_id: 0,
            next_outgoing_message_id: 0,
            attached_entity: None,
        };
    }

    fn generate_message_id(&mut self) -> u16 {
        let id = self.next_outgoing_message_id;
        self.next_outgoing_message_id = self.next_outgoing_message_id.wrapping_add(1);
        return id;
    }

    pub fn send(&mut self, socket: &UdpSocket, address: &SocketAddr, message: &mut Message) {
        if self.is_connected() {
            let id = if message.has_id() {
                let generated_id = self.generate_message_id();
                message.set_id(generated_id);
                Some(generated_id)
            } else {
                None
            };

            let encoded = message.encode();

            if let Err(error) = send(socket, address, &encoded) {
                self.disconnect(error);
            } else if let Some(id) = id {
                self.unacknowledged_messages.insert(
                    id,
                    UnacknowledgedMessage {
                        data: encoded,
                        last_sent: Instant::now(),
                    },
                );
            }
        }
    }

    pub fn resend_unacknowledged_messages(
        &mut self,
        socket: &UdpSocket,
        address: &SocketAddr,
        time: Duration,
    ) {
        if self.is_connected() {
            for message in self.unacknowledged_messages.values_mut() {
                if message.last_sent.elapsed() > time {
                    message.last_sent = Instant::now();

                    if let Err(error) = send(socket, address, &message.data) {
                        self.disconnect(error);
                        break;
                    }
                }
            }
        }
    }

    pub fn filter_message(&mut self, message: Message) -> Option<Message> {
        if let Some(id) = message.get_id() {
            match id.cmp(&self.next_incoming_message_id) {
                Ordering::Greater => {
                    self.held_messages.insert(id, message);
                    return None;
                }
                Ordering::Less => {
                    return None;
                }
                Ordering::Equal => {
                    self.next_incoming_message_id = self.next_incoming_message_id.wrapping_add(1);
                    return Some(message);
                }
            }
        } else {
            return Some(message);
        }
    }

    pub fn take_next_held_messages(&mut self) -> Vec<Message> {
        let mut messages = Vec::new();

        while let Some(message) = self.held_messages.remove(&self.next_incoming_message_id) {
            messages.push(message);
            self.next_incoming_message_id = self.next_incoming_message_id.wrapping_add(1);
        }

        return messages;
    }

    pub fn acknowledge_message(&mut self, id: u16) {
        self.unacknowledged_messages.remove(&id);
    }

    pub fn disconnect(&mut self, reason: String) {
        if self.is_connected() {
            self.unacknowledged_messages = HashMap::new();
            self.held_messages = HashMap::new();
            self.status = NetConnectionStatus::Disconnected(reason);
        }
    }

    pub const fn get_status(&self) -> &NetConnectionStatus {
        return &self.status;
    }

    pub const fn is_connected(&self) -> bool {
        return matches!(self.status, NetConnectionStatus::Connected);
    }
}

// TODO: avoid
fn send(socket: &UdpSocket, address: &SocketAddr, message: &[u8]) -> Result<usize, String> {
    return socket
        .send_to(message, address)
        .map_err(|e| format!("{}", e));
}
