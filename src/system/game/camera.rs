use crate::{component::Player, data::VIEW_DISTANCE, util::ext::TransformExt};
use bevy::{
    ecs::system::Query,
    math::{Quat, Vec2, Vec3},
    prelude::{Camera, OrthographicProjection, Transform, With, Without},
    window::{PrimaryWindow, Window},
};
use std::f32::consts::FRAC_PI_2;

const OFFSET_RATIO: f32 = 0.25;

pub fn camera(
    mut cameras: Query<(&mut Transform, &mut OrthographicProjection)>,
    players: Query<
        &Transform,
        (
            With<Player>,
            Without<Camera>,
            Without<OrthographicProjection>,
        ),
    >,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window_size = if let Some(window) = windows.iter().next() {
        Vec2::new(window.width(), window.height())
    } else {
        return;
    };

    if let Some(player) = players.iter().next() {
        if let Some((mut transform, mut projection)) = cameras.iter_mut().next() {
            projection.scale = VIEW_DISTANCE / window_size.length();
            let rotation = Quat::from_rotation_z(player.direction() - FRAC_PI_2);
            let offset =
                rotation * Vec3::new(0.0, window_size.y * projection.scale * OFFSET_RATIO, 0.0);
            transform.translation.x = player.translation.x + offset.x;
            transform.translation.y = player.translation.y + offset.y;
            transform.rotation = rotation;
        }
    }
}
