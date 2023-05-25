use crate::{
    component::{Interpolation, Player},
    resource::ServerData,
};
use bevy::{
    math::Quat,
    prelude::{Query, Res, Time, Transform, Without},
};

pub fn interpolation(
    mut query: Query<(&Interpolation, &mut Transform), Without<Player>>,
    server_data: Res<ServerData>,
    time: Res<Time>,
) {
    let time = time.elapsed();
    let interpolation_duration = server_data.sync_interval;

    for (interpolation, mut transform) in query.iter_mut() {
        let interpolated = interpolation.get_interpolated_transform(interpolation_duration, time);
        transform.translation.x = interpolated.translation.x;
        transform.translation.y = interpolated.translation.y;
        transform.rotation = Quat::from_rotation_z(interpolated.direction);
    }
}
