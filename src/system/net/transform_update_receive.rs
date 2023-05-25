use crate::{
    component::{Interpolation, Player},
    model::geometry::GeometryProjection,
    resource::{ServerData, TransformUpdateResource},
};
use bevy::{
    ecs::entity::Entity,
    math::Vec3Swizzles,
    prelude::{Query, Res, ResMut, Time, Transform},
};

pub fn transform_update_receive(
    mut updates: ResMut<TransformUpdateResource>,
    mut query: Query<(&mut Transform, &mut Interpolation, Option<&Player>)>,
    server_data: Res<ServerData>,
    time: Res<Time>,
) {
    let time = time.elapsed();
    let sync_interval = server_data.sync_interval;

    while let Some((entity_index, update)) = updates.pop() {
        let entity = Entity::from_raw(entity_index);
        let mut ghost = None;

        if let Ok((mut transform, mut interpolation, player)) = query.get_mut(entity) {
            if let Some(player) = player {
                let p_player = transform.translation.xy();
                let p_server = update.translation;
                let p_server_previous = interpolation.target.translation;
                let path = (p_server_previous, p_player);
                let p_intermediate = p_server.project_on(&path);
                let diff = p_server - p_intermediate;
                transform.translation.x += diff.x;
                transform.translation.y += diff.y;
                ghost = player.ghost;
            }

            interpolation.next(update.into(), sync_interval, time);
        }

        if let Some((_, mut interpolation, _)) = ghost.and_then(|g| query.get_mut(g).ok()) {
            interpolation.next(update.into(), sync_interval, time);
        }
    }
}
