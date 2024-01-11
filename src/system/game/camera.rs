use crate::{component::Player, data::VIEW_DISTANCE, util::ext::TransformExt};
use bevy::{
    ecs::system::Query,
    math::{Quat, Vec2, Vec3},
    prelude::{Camera, OrthographicProjection, Transform, With, Without},
    window::{PrimaryWindow, Window},
};
use std::f32::consts::FRAC_PI_2;

const AIM_OFFSET_RATIO: f32 = 1.0 / 3.0;
const SHAKE_FACTOR_Y: f32 = 4.0;
const SHAKE_FACTOR_Z: f32 = 0.6;

pub fn camera(
    mut cameras: Query<(&mut Transform, &mut OrthographicProjection)>,
    players: Query<(&Player, &Transform), (Without<Camera>, Without<OrthographicProjection>)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window_size = if let Some(window) = windows.iter().next() {
        Vec2::new(window.width(), window.height())
    } else {
        return;
    };

    if let Some((player, player_transform)) = players.iter().next() {
        let shake_y = player.get_shake_abs() * SHAKE_FACTOR_Y;
        let shake_z = player.get_shake_abs() * SHAKE_FACTOR_Z * player.get_zoom();
        let zoom = player.get_zoom() - shake_z;
        let mut direction = player.get_shake() + player_transform.direction();

        if player.is_aiming {
            direction -= player.get_extra_rotation();
        }

        if let Some((mut transform, mut projection)) = cameras.iter_mut().next() {
            projection.scale = VIEW_DISTANCE / zoom / window_size.length();
            let rotation = Quat::from_rotation_z(direction - FRAC_PI_2);
            let mut offset_y = -shake_y;

            if player.is_aiming {
                offset_y += window_size.y * projection.scale * AIM_OFFSET_RATIO;
            }

            let offset = rotation * Vec3::new(0.0, offset_y, 0.0);

            transform.translation.x = player_transform.translation.x + offset.x;
            transform.translation.y = player_transform.translation.y + offset.y;

            if player.is_aiming {
                transform.rotation = rotation;
            }
        }
    }
}
