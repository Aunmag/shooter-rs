use crate::components::TransformSync;
use crate::resources::EntityIndexMap;
use crate::resources::Message;
use crate::resources::MessageReceiver;
use crate::resources::MessageResource;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::Entities;
use amethyst::ecs::ReadExpect;
use amethyst::ecs::Write;
use std::time::Duration;
use std::time::Instant;

const INTERVAL: Duration = Duration::from_secs(2); // TODO: Tweak

#[derive(SystemDesc)]
pub struct TransformSyncSystem {
    last_sync: Instant,
}

impl TransformSyncSystem {
    pub fn new() -> Self {
        return Self {
            last_sync: Instant::now(),
        };
    }
}

impl<'a> System<'a> for TransformSyncSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, EntityIndexMap>,
        ReadStorage<'a, TransformSync>,
        Write<'a, MessageResource>,
    );

    fn run(&mut self, (entities, id_map, transforms_sync, mut messages): Self::SystemData) {
        if self.last_sync.elapsed() < INTERVAL {
            return;
        }

        self.last_sync = Instant::now();

        for (entity, transform_sync) in (&entities, &transforms_sync).join() {
            if let Some(public_id) = id_map.to_public_id(entity.id()) {
                messages.push((
                    MessageReceiver::Every,
                    Message::TransformSync {
                        id: 0,
                        public_id,
                        x: transform_sync.target_x,
                        y: transform_sync.target_y,
                        angle: transform_sync.target_angle,
                    },
                ));
            }
        }
    }
}
