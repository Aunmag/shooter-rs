use crate::component::Player;
use crate::data::VIEW_DISTANCE;
use crate::util::ext::TransformExt;
use bevy::ecs::system::Query;
use bevy::math::Vec2;
use bevy::prelude::Camera;
use bevy::prelude::OrthographicProjection;
use bevy::prelude::Res;
use bevy::prelude::Transform;
use bevy::prelude::With;
use bevy::prelude::Without;
use bevy::window::Windows;
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
    windows: Res<Windows>,
) {
    let window_size;

    if let Some(window) = windows.get_primary() {
        window_size = Vec2::new(window.width(), window.height());
    } else {
        return;
    }

    if let Some(player) = players.iter().next() {
        if let Some((mut transform, mut projection)) = cameras.iter_mut().next() {
            let scale = VIEW_DISTANCE / window_size.length();
            let view = window_size * scale;
            let offset = window_size.y * scale * OFFSET_RATIO;
            projection.top = view.y;
            projection.left = -view.x;
            projection.bottom = -view.y;
            projection.right = view.x;
            projection.scale = scale;

            let (sin, cos) = (player.direction() + FRAC_PI_2).sin_cos();
            transform.translation.x = player.translation.x + offset * cos;
            transform.translation.y = player.translation.y + offset * sin;
            transform.rotation = player.rotation;
        }
    }
}
