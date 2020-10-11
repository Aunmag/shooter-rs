use crate::tools::net::message::Message;
use crate::tools::net::message::MessageContainer;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::time::Duration;
use std::time::Instant;

const MESSAGE_RESEND_INTERVAL: Duration = Duration::from_millis(500);

pub struct Connection<R: Message> {
    pub attached_entity_id: u16,
    unacknowledged_messages: HashMap<u16, UnacknowledgedMessage>,
    early_messages: HashMap<u16, R>, // TODO: Don't allow it grow to large
    next_incoming_message_id: u16,   // TODO: Handle ID restart
    next_outgoing_message_id: u16,
}

impl<R: Message> Connection<R> {
    pub fn new() -> Self {
        return Self {
            attached_entity_id: 0,
            unacknowledged_messages: HashMap::new(),
            early_messages: HashMap::new(),
            next_incoming_message_id: 0,
            next_outgoing_message_id: 0,
        };
    }

    pub fn update(&mut self, address: &SocketAddr, socket: &UdpSocket) {
        for (_id, message) in self.unacknowledged_messages.iter_mut() {
            message.resend_if_ready(address, socket);
        }
    }

    pub fn send<S: Message>(
        &mut self,
        address: &SocketAddr,
        message: &mut MessageContainer<S>,
        socket: &UdpSocket,
    ) {
        match *message {
            MessageContainer::Decoded(ref mut message) => {
                let id = self.generate_message_id();
                message.set_id(id);

                let encoded = message.encode();
                socket.send_to(&encoded, address).unwrap();

                self.unacknowledged_messages
                    .insert(id, UnacknowledgedMessage::new(encoded));
            }
            MessageContainer::Encoded(ref encoded) => {
                socket.send_to(encoded, address).unwrap();
            }
        }
    }

    fn generate_message_id(&mut self) -> u16 {
        let id = self.next_outgoing_message_id;
        self.next_outgoing_message_id = self.next_outgoing_message_id.wrapping_add(1);
        return id;
    }

    pub fn filter_message(&mut self, message: R) -> Option<R> {
        if let Some(id) = message.get_id() {
            match id.cmp(&self.next_incoming_message_id) {
                Ordering::Greater => {
                    self.early_messages.insert(id, message);
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

    // TODO: Maybe find a better name
    pub fn take_held_message(&mut self) -> Option<R> {
        let message = self.early_messages.remove(&self.next_incoming_message_id);

        if message.is_some() {
            self.next_incoming_message_id = self.next_incoming_message_id.wrapping_add(1);
        }

        return message;
    }

    pub fn acknowledge_message(&mut self, id: u16) {
        if self.unacknowledged_messages.remove(&id).is_none() {
            println!(
                "Got response for {} message but it was not a pending message",
                id,
            );
        }
    }
}

struct UnacknowledgedMessage {
    data: Vec<u8>,
    sent: Instant,
}

impl UnacknowledgedMessage {
    pub fn new(data: Vec<u8>) -> Self {
        return Self {
            data,
            sent: Instant::now(),
        };
    }

    pub fn resend_if_ready(&mut self, address: &SocketAddr, socket: &UdpSocket) {
        if self.sent.elapsed() >= MESSAGE_RESEND_INTERVAL {
            socket.send_to(&self.data, address).unwrap();
            self.sent = Instant::now();
        }
    }
}
