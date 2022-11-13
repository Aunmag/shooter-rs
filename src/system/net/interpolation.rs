use crate::component::Interpolation;
use crate::component::Player;
use crate::resource::Config;
use bevy::math::Quat;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use bevy::prelude::Without;

pub fn interpolation(
    mut query: Query<(&Interpolation, &mut Transform), Without<Player>>,
    config: Res<Config>,
    time: Res<Time>,
) {
    let time = time.time_since_startup();
    let interpolation_duration = config.net.server.sync_interval; // TODO: don't get from config, get from server instead

    for (interpolation, mut transform) in query.iter_mut() {
        let interpolated = interpolation.get_interpolated_position(interpolation_duration, time);
        transform.translation.x = interpolated.x;
        transform.translation.y = interpolated.y;
        transform.rotation = Quat::from_rotation_z(interpolated.direction);
    }
}
