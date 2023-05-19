use crate::component::Player;
use crate::data::VIEW_DISTANCE;
use crate::util::ext::TransformExt;
use bevy::ecs::system::Query;
use bevy::math::Vec2;
use bevy::prelude::Camera;
use bevy::prelude::OrthographicProjection;
use bevy::prelude::Transform;
use bevy::prelude::With;
use bevy::prelude::Without;
use bevy::window::PrimaryWindow;
use bevy::window::Window;
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
            let offset = window_size.y * projection.scale * OFFSET_RATIO;
            let direction = player.direction() + FRAC_PI_2;
            transform.translation.x = player.translation.x + offset * direction.cos();
            transform.translation.y = player.translation.y + offset * direction.sin();
            transform.rotation = player.rotation;
        }
    }
}
