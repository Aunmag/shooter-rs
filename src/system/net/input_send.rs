use crate::component::Actor;
use crate::component::ActorActions;
use crate::component::Player;
use crate::resource::Message;
use crate::resource::NetResource;
use crate::util::ext::TransformExt;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use bevy::prelude::With;
use std::time::Duration;

/// 25 Hz
const DIRECTION_SEND_INTERVAL_ACTIVE: Duration = Duration::from_millis(40); // TODO: from server

/// 10 Hz
const DIRECTION_SEND_INTERVAL_PASSIVE: Duration = Duration::from_millis(100);

#[derive(Default)]
pub struct InputSendData {
    time: Duration,
    actions: ActorActions,
    direction: f32,
}

pub fn input_send(
    query: Query<(&Actor, &Transform), With<Player>>,
    mut previous: ResMut<InputSendData>,
    mut net: ResMut<NetResource>,
    time: Res<Time>,
) {
    if let Some((actor, transform)) = query.iter().next() {
        let current = InputSendData {
            time: time.time_since_startup(),
            actions: actor.actions,
            direction: transform.direction(),
        };

        let message;

        #[allow(clippy::float_cmp, clippy::if_not_else)]
        if current.actions != previous.actions {
            message = Some(Message::ClientInput {
                id: 0,
                actions: current.actions.bits(),
                direction: current.direction,
            });
        } else if current.direction != previous.direction {
            let interval;

            if current.actions.is_empty() {
                interval = DIRECTION_SEND_INTERVAL_PASSIVE;
            } else {
                interval = DIRECTION_SEND_INTERVAL_ACTIVE;
            }

            if current.time.saturating_sub(previous.time) > interval {
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
            *previous = current; // TODO: make sure it works
        }
    }
}
