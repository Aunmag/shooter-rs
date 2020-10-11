use crate::components::Actor;
use crate::resources::EntityIndexMap;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use crate::resources::ServerMessageResource;
use crate::systems::NetworkSystem;
use crate::tools::net::message::ClientMessage;
use crate::tools::net::message::ServerMessage;
use crate::tools::net::postman::Postman;
use crate::tools::net::postman::Receiver;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::Entities;
use amethyst::ecs::Write;
use std::net::SocketAddr;
use std::net::UdpSocket;

#[derive(SystemDesc)]
pub struct ServerSystem {
    postman: Postman<ClientMessage>,
}

impl ServerSystem {
    pub fn new(port: u16) -> Result<Self, String> {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).map_err(|e| format!("{}", e))?;
        socket.set_nonblocking(true).map_err(|e| format!("{}", e))?;

        return Ok(Self {
            postman: Postman::new(socket),
        });
    }

    fn on_connect(&mut self, address: SocketAddr, data: &mut ServerSystemData) {
        for (entity, _, transform) in (&data.entities, &data.actors, &data.transforms).join() {
            let entity_id = data.id_map.to_external_or_generate(entity.id());

            self.postman.send(
                ServerMessage::ActorSpawn {
                    id: 0,
                    entity_id,
                    // TODO: Pass transform sync
                    x: transform.translation().x,
                    y: transform.translation().y,
                    angle: transform.euler_angles().2,
                },
                &Receiver::Only(address),
            );
        }

        let entity_id = data.id_map.generate();
        self.postman.attach_entity_id(address, entity_id);

        self.postman.send(
            ServerMessage::ActorSpawn {
                id: 0,
                entity_id,
                x: 0.0,
                y: 0.0,
                angle: 0.0,
            },
            &Receiver::Every,
        );

        self.postman.send(
            ServerMessage::ActorGrant { id: 0, entity_id },
            &Receiver::Only(address),
        );

        data.tasks.push(GameTask::ActorSpawn {
            entity_id,
            x: 0.0,
            y: 0.0,
            angle: 0.0,
        });
    }
}

impl NetworkSystem<ServerSystemData<'_>, ServerMessage, ClientMessage> for ServerSystem {
    fn on_message(
        &mut self,
        address: SocketAddr,
        message: &ClientMessage,
        data: &mut ServerSystemData,
    ) {
        match *message {
            ClientMessage::Response { message_id } => {
                self.postman.acknowledge_message(address, message_id);
            }
            ClientMessage::Greeting { .. } => {
                self.on_connect(address, data);
            }
            ClientMessage::ActorAction {
                move_x,
                move_y,
                angle,
                ..
            } => {
                let entity_id = self.postman.get_attached_entity_id(address);

                if entity_id != 0 {
                    data.tasks.push(GameTask::ActorAction {
                        entity_id,
                        move_x,
                        move_y,
                        angle,
                    });

                    self.postman.send(
                        ServerMessage::ActorAction {
                            id: 0,
                            entity_id,
                            move_x,
                            move_y,
                            angle,
                        },
                        &Receiver::Every,
                    );
                }
            }
        }
    }

    fn get_postman_mut(&mut self) -> &mut Postman<ClientMessage> {
        return &mut self.postman;
    }
}

impl<'a> System<'a> for ServerSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Actor>,
        ReadStorage<'a, Transform>,
        Write<'a, EntityIndexMap>,
        Write<'a, GameTaskResource>,
        Write<'a, Option<ServerMessageResource>>,
    );

    fn run(&mut self, (entities, actors, transforms, id_map, tasks, messages): Self::SystemData) {
        let mut data = ServerSystemData {
            entities,
            actors,
            transforms,
            id_map,
            tasks,
            messages,
        };

        NetworkSystem::run(self, &mut data);

        if let Some(messages) = data.messages.as_mut() {
            for message in messages.drain(..) {
                self.postman.send(message, &Receiver::Every);
            }
        }
    }
}

struct ServerSystemData<'a> {
    entities: Entities<'a>,
    actors: ReadStorage<'a, Actor>,
    transforms: ReadStorage<'a, Transform>,
    id_map: Write<'a, EntityIndexMap>,
    tasks: Write<'a, GameTaskResource>,
    messages: Write<'a, Option<ServerMessageResource>>,
}
