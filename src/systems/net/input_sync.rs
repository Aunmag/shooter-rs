use crate::components::Player;
use crate::resources::EntityIndexMap;
use crate::resources::Message;
use crate::resources::MessageReceiver;
use crate::resources::MessageResource;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::ReadExpect;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::WriteStorage;
use amethyst::ecs::Entities;
use amethyst::ecs::Write;
use std::time::Duration;
use std::time::Instant;

#[allow(clippy::integer_division)]
pub const INTERVAL: Duration = Duration::from_millis(1000 / 25); // TODO: Tweak

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
        ReadExpect<'a, EntityIndexMap>,
        ReadStorage<'a, Transform>,
        Write<'a, MessageResource>,
        WriteStorage<'a, Player>,
    );

    fn run(&mut self, (entities, id_map, transforms, mut messages, mut players): Self::SystemData) {
        if self.last_sync.elapsed() < INTERVAL {
            return;
        }

        self.last_sync = Instant::now();

        for (entity, transform, player) in (&entities, &transforms, &mut players).join() {
            let (movement_x, movement_y, rotation) = player.accumulated_input.take();

            if movement_x != 0.0 || movement_y != 0.0 || rotation != 0.0 {
                let angle = transform.euler_angles().2;

                if let Some(public_id) = id_map.to_public_id(entity.id()) {
                    messages.push((
                        MessageReceiver::Every,
                        Message::ActorAction {
                            id: 0,
                            public_id,
                            move_x: movement_x,
                            move_y: movement_y,
                            angle,
                        },
                    ));
                }
            }
        }
    }
}
