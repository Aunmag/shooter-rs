use crate::component::Actor;
use crate::model::Position;
use crate::resource::Message;
use crate::resource::NetResource;
use crate::util::Timer;
use bevy::prelude::Entity;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use bevy::prelude::With;
use std::collections::HashMap;
use std::time::Duration;

pub struct PositionUpdateSendData {
    timer: Timer,
    cache: HashMap<u32, Position>,
}

impl PositionUpdateSendData {
    pub fn new(sync_interval: Duration) -> Self {
        return Self {
            timer: Timer::new(sync_interval),
            cache: HashMap::new(),
        };
    }
}

pub fn position_update_send(
    query: Query<(Entity, &Transform), With<Actor>>,
    mut data: ResMut<PositionUpdateSendData>,
    time: Res<Time>,
    net: Res<NetResource>,
) {
    if net.connections.is_empty() {
        return;
    }

    if !data.timer.next_if_done(time.time_since_startup()) {
        return;
    }

    for (entity, transform) in query.iter() {
        let position = Position::from(transform);
        let entity_id = entity.id();

        if data.cache.get(&entity_id).map_or(true, |p| p != &position) {
            net.send_unreliably_to_all(&Message::PositionUpdate {
                entity_id,
                position,
            });

            data.cache.insert(entity_id, position);
        }
    }
}
