use crate::components::Player;
use crate::resources::ClientMessageResource;
use crate::resources::EntityIndexMap;
use crate::resources::ServerMessageResource;
use crate::tools::net::message::ClientMessage;
use crate::tools::net::message::ServerMessage;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::WriteStorage;
use amethyst::ecs::Entities;
use amethyst::ecs::Write;
use std::time::Duration;
use std::time::Instant;

#[allow(clippy::integer_division)]
const INTERVAL: Duration = Duration::from_millis(1000 / 25); // TODO: Tweak

#[derive(SystemDesc)]
pub struct InputSyncSystem {
    last_sync: Instant,
}

impl InputSyncSystem {
    pub fn new() -> Self {
        return Self {
            last_sync: Instant::now(),
        };
    }
}

impl<'a> System<'a> for InputSyncSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        Write<'a, EntityIndexMap>,
        Write<'a, Option<ClientMessageResource>>,
        Write<'a, Option<ServerMessageResource>>,
        WriteStorage<'a, Player>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            mut id_map,
            mut client_messages,
            mut server_messages,
            mut players,
        ): Self::SystemData,
    ) {
        if self.last_sync.elapsed() > INTERVAL {
            self.last_sync = Instant::now();

            for (entity, transform, player) in (&entities, &transforms, &mut players).join() {
                let (movement_x, movement_y, rotation) = player.accumulated_input.take();

                if movement_x != 0.0 || movement_y != 0.0 || rotation != 0.0 {
                    let angle = transform.euler_angles().2;

                    if let Some(client_messages) = client_messages.as_mut() {
                        client_messages.push(ClientMessage::ActorAction {
                            id: 0,
                            move_x: movement_x,
                            move_y: movement_y,
                            angle,
                        });
                    }

                    if let Some(server_messages) = server_messages.as_mut() {
                        server_messages.push(ServerMessage::ActorAction {
                            id: 0,
                            entity_id: id_map.to_external_or_generate(entity.id()),
                            move_x: movement_x,
                            move_y: movement_y,
                            angle,
                        });
                    }
                }
            }
        }
    }
}
