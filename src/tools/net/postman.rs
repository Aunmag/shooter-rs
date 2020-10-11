use crate::tools::net::connection::Connection;
use crate::tools::net::message::Message;
use crate::tools::net::message::MessageContainer;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::net::UdpSocket;

const BUFFER_SIZE: usize = 128; // TODO: Tweak

pub struct Postman<R: Message> {
    socket: UdpSocket,
    connections: HashMap<SocketAddr, Connection<R>>,
}

impl<R: Message> Postman<R> {
    pub fn new(socket: UdpSocket) -> Self {
        return Self {
            socket,
            connections: HashMap::new(),
        };
    }

    pub fn update(&mut self) {
        for (address, connection) in self.connections.iter_mut() {
            connection.update(&address, &self.socket);
        }
    }

    pub fn send<S: Message>(&mut self, message: S, receiver: &Receiver) {
        let mut message_container;

        if message.has_id() {
            message_container = MessageContainer::Decoded(message);
        } else {
            message_container = MessageContainer::Encoded(message.encode());
        }

        match *receiver {
            Receiver::Every => {
                for (address, connection) in self.connections.iter_mut() {
                    connection.send(&address, &mut message_container, &self.socket);
                }
            }
            Receiver::Only(ref address) => {
                if let Some(connection) = self.connections.get_mut(address) {
                    connection.send(address, &mut message_container, &self.socket);
                }
            }
        }
    }

    pub fn pull_messages<S: Message>(&mut self) -> Vec<(SocketAddr, R)> {
        let mut messages = Vec::new();

        loop {
            let mut buffer = [0; BUFFER_SIZE];

            match self.socket.recv_from(&mut buffer) {
                Ok((message_length, address)) => {
                    #[allow(clippy::indexing_slicing)] // TODO: Resolve
                    match R::decode(&buffer[..message_length]) {
                        Ok(message) => {
                            let connection = self.keep_connection(address);
                            let message_id = message.get_id();
                            let message_filtered = connection.filter_message(message);

                            if let Some(id) = message_id {
                                self.send(S::into_response(id), &Receiver::Only(address));
                            }

                            if let Some(message) = message_filtered {
                                messages.push((address, message));
                            }
                        }
                        Err(error) => {
                            println!("[NET] error: {:?}", error); // TODO: Handle error properly
                        }
                    }
                }
                Err(error) => {
                    if error.kind() == ErrorKind::WouldBlock {
                        break;
                    } else {
                        println!("[NET] error: {:?}", error); // TODO: Handle error properly
                    }
                }
            }
        }

        for (address, connection) in self.connections.iter_mut() {
            while let Some(message) = connection.take_held_message() {
                messages.push((*address, message));
            }
        }

        return messages;
    }

    pub fn keep_connection(&mut self, address: SocketAddr) -> &mut Connection<R> {
        return self
            .connections
            .entry(address)
            .or_insert_with(Connection::<R>::new);
    }

    pub fn acknowledge_message(&mut self, address: SocketAddr, message_id: u16) {
        if let Some(connection) = self.connections.get_mut(&address) {
            connection.acknowledge_message(message_id);
        }
    }

    pub fn attach_entity_id(&mut self, address: SocketAddr, entity_id: u16) {
        if let Some(connection) = self.connections.get_mut(&address) {
            connection.attached_entity_id = entity_id;
        }
    }

    pub fn get_attached_entity_id(&self, address: SocketAddr) -> u16 {
        return self
            .connections
            .get(&address)
            .map_or(0, |c| c.attached_entity_id);
    }
}

pub enum Receiver {
    Every,
    Only(SocketAddr),
}
