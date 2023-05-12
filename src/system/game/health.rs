use crate::command::EntityDelete;
use crate::component::Health;
use bevy::ecs::system::Query;
use bevy::prelude::Commands;
use bevy::prelude::Entity;
use bevy::prelude::Res;
use bevy::prelude::Time;

pub fn health(query: Query<(Entity, &Health)>, time: Res<Time>, mut commands: Commands) {
    let now = time.elapsed();

    for (entity, health) in query.iter() {
        if health.is_decayed(now) {
            commands.add(EntityDelete(entity));
        }
    }
}
