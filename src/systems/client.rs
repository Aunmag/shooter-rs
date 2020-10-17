use crate::resources::ClientMessageResource;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use crate::systems::NetworkSystem;
use crate::tools::net::message::ClientMessage;
use crate::tools::net::message::ServerMessage;
use crate::tools::net::postman::Postman;
use crate::tools::net::postman::Receiver;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::Write;
use std::net::SocketAddr;
use std::net::UdpSocket;

#[derive(SystemDesc)]
pub struct ClientSystem {
    postman: Postman<ServerMessage>,
}

impl ClientSystem {
    pub fn new(server_address: SocketAddr) -> Result<Self, String> {
        let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| format!("{}", e))?;
        socket.set_nonblocking(true).map_err(|e| format!("{}", e))?;

        let mut client = Self {
            postman: Postman::new(socket),
        };

        client.postman.keep_connection(server_address);
        client
            .postman
            .send(ClientMessage::Greeting { id: 0 }, &Receiver::Every);

        return Ok(client);
    }
}

impl NetworkSystem<ClientSystemData<'_>, ClientMessage, ServerMessage> for ClientSystem {
    fn on_message(
        &mut self,
        address: SocketAddr,
        message: &ServerMessage,
        data: &mut ClientSystemData,
    ) {
        match *message {
            ServerMessage::Response { message_id } => {
                self.postman.acknowledge_message(address, message_id);
            }
            ServerMessage::ActorSpawn {
                public_id,
                x,
                y,
                angle,
                ..
            } => {
                data.tasks.push(GameTask::ActorSpawn {
                    public_id,
                    x,
                    y,
                    angle,
                });
            }
            ServerMessage::ActorGrant { public_id, .. } => {
                data.tasks.push(GameTask::ActorGrant(public_id));
            }
            ServerMessage::ActorAction {
                public_id,
                move_x,
                move_y,
                angle,
                ..
            } => {
                data.tasks.push(GameTask::ActorAction {
                    public_id,
                    move_x,
                    move_y,
                    angle,
                });
            }
            ServerMessage::TransformSync {
                public_id,
                x,
                y,
                angle,
                ..
            } => {
                data.tasks.push(GameTask::TransformSync {
                    public_id,
                    x,
                    y,
                    angle,
                });
            }
        }
    }

    fn get_postman_mut(&mut self) -> &mut Postman<ServerMessage> {
        return &mut self.postman;
    }
}

impl<'a> System<'a> for ClientSystem {
    type SystemData = (
        Write<'a, GameTaskResource>,
        Write<'a, Option<ClientMessageResource>>,
    );

    fn run(&mut self, (tasks, messages): Self::SystemData) {
        let mut data = ClientSystemData { tasks, messages };

        NetworkSystem::run(self, &mut data);

        if let Some(messages) = data.messages.as_mut() {
            for message in messages.drain(..) {
                self.postman.send(message, &Receiver::Every);
            }
        }
    }
}

struct ClientSystemData<'a> {
    tasks: Write<'a, GameTaskResource>,
    messages: Write<'a, Option<ClientMessageResource>>,
}
