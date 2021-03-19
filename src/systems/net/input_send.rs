use crate::components::Actor;
use crate::components::ActorActions;
use crate::components::Player;
use crate::resources::Message;
use crate::resources::NetResource;
use crate::utils::DurationExt;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::Join;
use amethyst::ecs::Read;
use amethyst::ecs::ReadStorage;
use amethyst::ecs::System;
use amethyst::ecs::Write;
use std::time::Duration;

/// 25 Hz
const DIRECTION_SEND_INTERVAL_ACTIVE: Duration = Duration::from_millis(40);

/// 10 Hz
const DIRECTION_SEND_INTERVAL_PASSIVE: Duration = Duration::from_millis(100);

pub struct InputSendSystem {
    previous: InputSend,
}

struct InputSend {
    time: Duration,
    actions: ActorActions,
    direction: f32,
}

impl InputSendSystem {
    pub fn new() -> Self {
        return Self {
            previous: InputSend {
                time: Duration::from_millis(0),
                actions: ActorActions::empty(),
                direction: 0.0,
            },
        };
    }
}

impl<'a> System<'a> for InputSendSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Actor>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        Option<Write<'a, NetResource>>,
    );

    fn run(&mut self, (time, actors, players, transforms, net): Self::SystemData) {
        let mut net = match net {
            Some(net) => net,
            None => return,
        };

        #[allow(clippy::never_loop)]
        for (_, actor, transform) in (&players, &actors, &transforms).join() {
            let current = InputSend {
                time: time.absolute_real_time(),
                actions: actor.actions,
                direction: transform.euler_angles().2,
            };

            let message;

            #[allow(clippy::float_cmp, clippy::if_not_else)]
            if current.actions != self.previous.actions {
                message = Some(Message::ClientInput {
                    id: 0,
                    actions: current.actions.bits(),
                    direction: current.direction,
                });
            } else if current.direction != self.previous.direction {
                let interval;

                if current.actions.is_empty() {
                    interval = DIRECTION_SEND_INTERVAL_PASSIVE;
                } else {
                    interval = DIRECTION_SEND_INTERVAL_ACTIVE;
                }

                if current.time.sub_safely(self.previous.time) > interval {
                    message = Some(Message::ClientInputDirection {
                        id: 0,
                        direction: current.direction,
                    });
                } else {
                    message = None;
                }
            } else {
                message = None;
            }

            if let Some(message) = message {
                net.send_to_all(message);
                self.previous = current;
            }

            break; // since there should be only one player
        }
    }
}
