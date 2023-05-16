use crate::component::Interpolation;
use crate::component::Player;
use crate::resource::ServerData;
use bevy::math::Quat;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use bevy::prelude::Without;

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
