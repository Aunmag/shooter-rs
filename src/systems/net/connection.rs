use crate::resources::Message;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::time::Duration;
use std::time::Instant;

const MESSAGE_RESEND_INTERVAL: Duration = Duration::from_millis(400); // TODO: Tweak

pub struct Connection {
    status: ConnectionStatus,
    // TODO: Maybe don't allow grow to large
    unacknowledged_messages: HashMap<u16, UnacknowledgedMessage>,
    // TODO: Maybe don't allow grow to large
    held_messages: HashMap<u16, Message>,
    // TODO: Handle ID restart
    next_incoming_message_id: u16,
    next_outgoing_message_id: u16,
    pub attached_external_id: Option<u16>,
}

pub enum ConnectionStatus {
    Connected,
    Disconnected(String),
}

struct UnacknowledgedMessage {
    data: Vec<u8>,
    last_sent: Instant,
}

impl Connection {
    pub fn new() -> Self {
        return Self {
            status: ConnectionStatus::Connected,
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
            self.status = ConnectionStatus::Disconnected(reason);
        }
    }

    pub fn get_status(&self) -> &ConnectionStatus {
        return &self.status;
    }

    pub fn is_connected(&self) -> bool {
        return match self.status {
            ConnectionStatus::Connected => true,
            ConnectionStatus::Disconnected(..) => false,
        };
    }
}

fn send(socket: &UdpSocket, address: &SocketAddr, message: &[u8]) -> Result<usize, String> {
    return socket
        .send_to(message, address)
        .map_err(|e| format!("{}", e));
}
