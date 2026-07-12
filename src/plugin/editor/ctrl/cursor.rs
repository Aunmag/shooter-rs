use bevy::{ecs::{query::With, system::{Query, ResMut, Resource}}, math::Vec2, render::camera::Camera, transform::components::GlobalTransform, window::{PrimaryWindow, Window}};

use crate::plugin::MainCamera;

#[derive(Default, Resource)]
pub struct Cursor {
    pub p: Option<Vec2>,
}

impl Cursor {
    pub fn system(
        mut cursor: ResMut<Self>,
        windows: Query<&Window, With<PrimaryWindow>>,
        cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
        cursor.p = None;

        if let Some(cursor_on_screen) = windows.iter().next().and_then(|w| w.cursor_position()) {
            cursor.p = cameras
                .iter()
                .next()
                .and_then(|(c, t)| c.viewport_to_world(t, cursor_on_screen))
                .map(|v| v.origin.truncate());
        }
    }
}
