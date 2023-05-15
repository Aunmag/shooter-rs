use crate::component::Actor;
use crate::component::ActorAction;
use crate::component::Player;
use crate::resource::Message;
use crate::resource::NetResource;
use crate::util::ext::TransformExt;
use bevy::ecs::system::Resource;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use bevy::prelude::With;
use enumset::EnumSet;
use std::time::Duration;

/// 25 Hz
const DIRECTION_SEND_INTERVAL_ACTIVE: Duration = Duration::from_millis(40); // TODO: get from server

/// 10 Hz
const DIRECTION_SEND_INTERVAL_PASSIVE: Duration = Duration::from_millis(100);

#[derive(Default, Resource)]
pub struct InputSendData {
    time: Duration,
    actions: EnumSet<ActorAction>,
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
            time: time.elapsed(),
            actions: actor.actions,
            direction: transform.direction(),
        };

        let message;

        #[allow(clippy::float_cmp, clippy::if_not_else)]
        if current.actions != previous.actions {
            message = Some(Message::ClientInput {
                id: 0,
                actions: current.actions,
                direction: current.direction,
            });
        } else if current.direction != previous.direction {
            let interval = if current.actions.is_empty() {
                DIRECTION_SEND_INTERVAL_PASSIVE
            } else {
                DIRECTION_SEND_INTERVAL_ACTIVE
            };

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
