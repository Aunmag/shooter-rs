use crate::components::Actor;
use crate::components::ActorActions;
use crate::components::Player;
use crate::resources::Message;
use crate::resources::NetResource;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::WriteExpect;
use std::time::Duration;
use std::time::Instant;

#[allow(clippy::integer_division)]
pub const DIRECTION_SEND_INTERVAL_ACTIVE: Duration = Duration::from_millis(1000 / 60);

#[allow(clippy::integer_division)]
pub const DIRECTION_SEND_INTERVAL_PASSIVE: Duration = Duration::from_millis(1000 / 10);

#[derive(SystemDesc)]
pub struct InputSendSystem {
    // TODO: It may spam input messages if there will be more than one player, find a way to fix it
    last_actions: ActorActions,
    last_direction: f32,
    last_direction_send: Instant,
}

impl InputSendSystem {
    pub fn new() -> Self {
        return Self {
            last_actions: ActorActions::empty(),
            last_direction: 0.0,
            last_direction_send: Instant::now(),
        };
    }
}

impl<'a> System<'a> for InputSendSystem {
    type SystemData = (
        ReadStorage<'a, Actor>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        WriteExpect<'a, NetResource>,
    );

    fn run(&mut self, (actors, players, transforms, mut net): Self::SystemData) {
        #[allow(clippy::float_cmp)]
        for (_, actor, transform) in (&players, &actors, &transforms).join() {
            let direction = transform.euler_angles().2;

            if self.last_actions != actor.actions {
                net.send_to_all(Message::ClientInput {
                    id: 0,
                    actions: actor.actions.bits(),
                    direction,
                });

                self.last_actions = actor.actions;
                self.last_direction = direction;
                self.last_direction_send = Instant::now();
            } else if self.last_direction != direction {
                let direction_sync_interval;

                if actor.actions.is_empty() {
                    direction_sync_interval = DIRECTION_SEND_INTERVAL_PASSIVE;
                } else {
                    direction_sync_interval = DIRECTION_SEND_INTERVAL_ACTIVE;
                }

                if self.last_direction_send.elapsed() > direction_sync_interval {
                    net.send_to_all(Message::ClientInputDirection { id: 0, direction });
                    self.last_direction = direction;
                    self.last_direction_send = Instant::now();
                }
            }
        }
    }
}
