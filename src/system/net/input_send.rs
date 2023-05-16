use crate::component::Actor;
use crate::component::Player;
use crate::model::ActorActions;
use crate::model::ActorActionsExt;
use crate::resource::Message;
use crate::resource::NetResource;
use crate::resource::ServerData;
use crate::util::ext::TransformExt;
use bevy::ecs::system::Resource;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use bevy::prelude::With;
use std::time::Duration;

#[derive(Default, Resource)]
pub struct InputSendData {
    time: Duration,
    actions: ActorActions,
    direction: f32,
}

pub fn input_send(
    query: Query<(&Actor, &Transform), With<Player>>,
    mut previous: ResMut<InputSendData>,
    mut net: ResMut<NetResource>,
    server_data: Res<ServerData>,
    time: Res<Time>,
) {
    let current = if let Some((actor, transform)) = query.iter().next() {
        InputSendData {
            time: time.elapsed(),
            actions: actor.actions.clean(),
            direction: transform.direction(),
        }
    } else {
        return;
    };

    let message;

    #[allow(clippy::float_cmp, clippy::if_not_else)]
    if current.actions != previous.actions {
        message = Message::ClientInput {
            id: 0,
            actions: current.actions,
            direction: current.direction,
        };
    } else if current.direction != previous.direction {
        let interval = if current.actions.is_empty() {
            server_data.sync_interval * 3
        } else {
            server_data.sync_interval
        };

        if current.time.saturating_sub(previous.time) > interval {
            message = Message::ClientInputDirection {
                id: 0,
                direction: current.direction,
            };
        } else {
            return;
        }
    } else {
        return;
    }

    net.send_to_all(message);
    *previous = current;
}
