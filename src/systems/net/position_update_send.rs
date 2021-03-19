use crate::components::Actor;
use crate::data::POSITION_UPDATE_INTERVAL;
use crate::resources::EntityMap;
use crate::resources::Message;
use crate::resources::NetResource;
use crate::utils::Position;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::Read;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::Entities;
use std::collections::HashMap;
use std::time::Instant;

pub struct PositionUpdateSendSystem {
    last_sent: Instant,
    cache: HashMap<u16, Position>,
}

impl PositionUpdateSendSystem {
    pub fn new() -> Self {
        return Self {
            last_sent: Instant::now(),
            cache: HashMap::new(),
        };
    }
}

impl<'a> System<'a> for PositionUpdateSendSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, EntityMap>,
        Option<Read<'a, NetResource>>,
        ReadStorage<'a, Actor>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, (entities, entity_map, net, actors, transforms): Self::SystemData) {
        if self.last_sent.elapsed() < POSITION_UPDATE_INTERVAL {
            return;
        }

        let net = match net {
            Some(net) => net,
            None => return,
        };

        for (entity, _, transform) in (&entities, &actors, &transforms).join() {
            if let Some(external_id) = entity_map.get_external_id(entity) {
                let position = Position::from(transform);

                if self
                    .cache
                    .get(&external_id)
                    .map_or(true, |p| p != &position)
                {
                    net.send_to_all_unreliably(&Message::PositionUpdate {
                        external_id,
                        position,
                    });

                    self.cache.insert(external_id, position);
                }
            }
        }

        self.last_sent = Instant::now();
    }
}
