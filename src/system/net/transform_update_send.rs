use crate::component::Actor;
use crate::model::TransformLiteU8;
use crate::resource::Message;
use crate::resource::NetResource;
use crate::util::Timer;
use bevy::ecs::system::Resource;
use bevy::prelude::Entity;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use bevy::prelude::With;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Resource)]
pub struct TransformUpdateSendData {
    timer: Timer,
    cache: HashMap<u32, TransformLiteU8>,
}

impl TransformUpdateSendData {
    pub fn new(sync_interval: Duration) -> Self {
        return Self {
            timer: Timer::new(sync_interval),
            cache: HashMap::new(),
        };
    }
}

pub fn transform_update_send(
    query: Query<(Entity, &Transform), With<Actor>>,
    mut data: ResMut<TransformUpdateSendData>,
    time: Res<Time>,
    net: Res<NetResource>,
) {
    if net.connections.is_empty() {
        return;
    }

    if !data.timer.next_if_done(time.elapsed()) {
        return;
    }

    for (entity, transform) in query.iter() {
        let transform = TransformLiteU8::from(transform);
        let entity_index = entity.index();

        if data
            .cache
            .get(&entity_index)
            .map_or(true, |p| p != &transform)
        {
            net.send_unreliably_to_all(&Message::TransformUpdate {
                entity_index,
                transform,
            });

            data.cache.insert(entity_index, transform);
        }
    }
}
