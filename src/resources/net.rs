use crate::resources::Message;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::time::Duration;
use std::time::Instant;

const MESSAGE_RESEND_INTERVAL: Duration = Duration::from_millis(400); // TODO: Tweak

pub struct NetResource {
    is_server: bool,
    pub socket: UdpSocket,
    pub connections: HashMap<SocketAddr, NetConnection>,
}

pub struct NetConnection {
    status: NetConnectionStatus,
    // TODO: Maybe don't allow grow to large
    unacknowledged_messages: HashMap<u16, UnacknowledgedMessage>,
    // TODO: Maybe don't allow grow to large
    held_messages: HashMap<u16, Message>,
    // TODO: Handle ID restart
    next_incoming_message_id: u16,
    next_outgoing_message_id: u16,
    pub attached_external_id: Option<u16>,
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
    pub fn new_as_server(port: u16) -> Result<Self, String> {
        return Self::new(&format!("0.0.0.0:{}", port), true);
    }

    pub fn new_as_client(server_address: SocketAddr) -> Result<Self, String> {
        let mut network = Self::new("0.0.0.0:0", false)?;

        network
            .connections
            .insert(server_address, NetConnection::new());

        network.send_to_all(Message::Greeting { id: 0 });

        return Ok(network);
    }

    fn new(address: &str, is_server: bool) -> Result<Self, String> {
        let socket = UdpSocket::bind(address).map_err(|e| format!("{}", e))?;
        socket.set_nonblocking(true).map_err(|e| format!("{}", e))?;

        return Ok(Self {
            is_server,
            socket,
            connections: HashMap::new(),
        });
    }

    pub fn update_connections(&mut self) {
        let mut disconnected = Vec::new();

        for (address, connection) in self.connections.iter_mut() {
            connection.resend_unacknowledged_messages(&self.socket, &address);

            if let NetConnectionStatus::Disconnected(ref reason) = *connection.get_status() {
                disconnected.push(*address);
                log::warn!("{} disconnected. {}", address, reason);
            }
        }

        for address in disconnected.iter() {
            self.connections.remove(address);
        }
    }

    pub fn send_to(&mut self, address: &SocketAddr, mut message: Message) {
        if let Some(connection) = self.connections.get_mut(address) {
            connection.send(&self.socket, address, &mut message);
        }
    }

    pub fn send_to_all(&mut self, mut message: Message) {
        for (address, connection) in self.connections.iter_mut() {
            connection.send(&self.socket, &address, &mut message);
        }
    }

    /// A faster way to send a message by skipping acknowledgement and error handling.
    pub fn send_to_all_unreliably(&self, message: &Message) {
        let encoded = message.encode();

        #[allow(unused_must_use)]
        for address in self.connections.keys() {
            send(&self.socket, address, &encoded);
        }
    }

    pub fn attach_external_id(&mut self, address: &SocketAddr, external_id: u16) {
        if let Some(connection) = self.connections.get_mut(address) {
            connection.attached_external_id.replace(external_id);
        }
    }

    pub fn is_server(&self) -> bool {
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
            attached_external_id: None,
        };
    }

    fn generate_message_id(&mut self) -> u16 {
        let id = self.next_outgoing_message_id;
        self.next_outgoing_message_id = self.next_outgoing_message_id.wrapping_add(1);
        return id;
    }

    pub fn send(&mut self, socket: &UdpSocket, address: &SocketAddr, message: &mut Message) {
        if self.is_connected() {
            let id;

            if message.has_id() {
                let generated_id = self.generate_message_id();
                message.set_id(generated_id);
                id = Some(generated_id);
            } else {
                id = None;
            }

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

    pub fn resend_unacknowledged_messages(&mut self, socket: &UdpSocket, address: &SocketAddr) {
        if self.is_connected() {
            for message in self.unacknowledged_messages.values_mut() {
                if message.last_sent.elapsed() > MESSAGE_RESEND_INTERVAL {
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
        if self.unacknowledged_messages.remove(&id).is_none() {
            log::warn!(
                "Got response for {} message but it was not an unacknowledged message",
                id,
            );
        }
    }

    pub fn disconnect(&mut self, reason: String) {
        if self.is_connected() {
            self.unacknowledged_messages = HashMap::new();
            self.held_messages = HashMap::new();
            self.status = NetConnectionStatus::Disconnected(reason);
        }
    }

    pub fn get_status(&self) -> &NetConnectionStatus {
        return &self.status;
    }

    pub fn is_connected(&self) -> bool {
        return match self.status {
            NetConnectionStatus::Connected => true,
            NetConnectionStatus::Disconnected(..) => false,
        };
    }
}

fn send(socket: &UdpSocket, address: &SocketAddr, message: &[u8]) -> Result<usize, String> {
    return socket
        .send_to(message, address)
        .map_err(|e| format!("{}", e));
}
