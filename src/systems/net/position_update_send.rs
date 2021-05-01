use crate::components::Actor;
use crate::data::POSITION_UPDATE_INTERVAL;
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
    cache: HashMap<u32, Position>,
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
        Read<'a, Time>,
        Option<Read<'a, NetResource>>,
        ReadStorage<'a, Actor>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, (entities, time, net, actors, transforms): Self::SystemData) {
        if !self.timer.next_if_done(time.absolute_real_time()) {
            return;
        }

        let net = match net {
            Some(net) => net,
            None => return,
        };

        for (entity, _, transform) in (&entities, &actors, &transforms).join() {
            let position = Position::from(transform);
            let entity_id = entity.id();

            if self.cache.get(&entity_id).map_or(true, |p| p != &position) {
                net.send_to_all_unreliably(&Message::PositionUpdate {
                    entity_id,
                    position,
                });

                self.cache.insert(entity_id, position);
            }
        }
    }
}
