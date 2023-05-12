use crate::component::Interpolation;
use crate::component::Player;
use crate::model::geometry::GeometryProjection;
use crate::resource::Config;
use crate::resource::PositionUpdateResource;
use bevy::ecs::entity::Entity;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Time;
use bevy::prelude::Transform;

pub fn position_update_receive(
    mut updates: ResMut<PositionUpdateResource>,
    mut query: Query<(&mut Transform, &mut Interpolation, Option<&Player>)>,
    config: Res<Config>,
    time: Res<Time>,
) {
    let time = time.elapsed();
    let sync_interval = config.net.server.sync_interval; // TODO: don't get from config, get from server instead

    while let Some((entity_index, update)) = updates.pop() {
        let entity = Entity::from_raw(entity_index);
        let mut ghost = None;

        if let Ok((mut transform, mut interpolation, player)) = query.get_mut(entity) {
            if let Some(player) = player {
                let p_player = transform.translation.xy();
                let p_server = update.xy();
                let p_server_previous = interpolation.target.xy();
                let path = (p_server_previous, p_player);
                let p_intermediate = p_server.project_on(&path);
                let diff = p_server - p_intermediate;
                transform.translation.x += diff.x;
                transform.translation.y += diff.y;
                ghost = player.ghost;
            }

            interpolation.next(update, sync_interval, time);
        }

        if let Some((_, mut interpolation, _)) = ghost.and_then(|g| query.get_mut(g).ok()) {
            interpolation.next(update, sync_interval, time);
        }
    }
}
