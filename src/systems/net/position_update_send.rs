use crate::components::Actor;
use crate::data::POSITION_UPDATE_INTERVAL;
use crate::resources::EntityMap;
use crate::resources::Message;
use crate::resources::NetResource;
use crate::utils::Position;
use crate::utils::Timer;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::Entities;
use amethyst::ecs::Join;
use amethyst::ecs::Read;
use amethyst::ecs::ReadStorage;
use amethyst::ecs::System;
use std::collections::HashMap;

pub struct PositionUpdateSendSystem {
    timer: Timer,
    cache: HashMap<u16, Position>,
}

impl PositionUpdateSendSystem {
    pub fn new() -> Self {
        return Self {
            timer: Timer::new(POSITION_UPDATE_INTERVAL),
            cache: HashMap::new(),
        };
    }
}

impl<'a> System<'a> for PositionUpdateSendSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, EntityMap>,
        Read<'a, Time>,
        Option<Read<'a, NetResource>>,
        ReadStorage<'a, Actor>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, (entities, entity_map, time, net, actors, transforms): Self::SystemData) {
        if !self.timer.next_if_done(time.absolute_real_time()) {
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
    }
}
