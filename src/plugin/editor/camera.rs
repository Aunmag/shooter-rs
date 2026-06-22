use crate::{plugin::MainCamera, util::ext::Vec2Ext};
use bevy::{
    app::Update,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{component::Component, event::EventReader, query::With, system::Query},
    input::{keyboard::KeyCode, mouse::MouseWheel, ButtonInput},
    math::{Quat, Vec2},
    prelude::{App, Plugin, Res, Time, World},
    render::camera::OrthographicProjection,
    transform::components::Transform,
    window::{PrimaryWindow, Window},
};

const MOVE_SPEED: f32 = 0.5;
const ZOOM_SPEED: f32 = 0.1;
const SPEED_UP_FACTOR: f32 = 3.0;

pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_update);
    }
}

impl EditorCameraPlugin {
    pub fn spawn_camera(world: &mut World) {
        world
            .spawn(Camera2dBundle {
                transform: Transform {
                    rotation: Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2), // look to the north
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(MainCamera)
            .insert(EditorCamera { view: 10.0 });
    }
}

#[derive(Component)]
struct EditorCamera {
    view: f32,
}

fn on_update(
    mut cameras: Query<(
        &mut Transform,
        &mut EditorCamera,
        &mut OrthographicProjection,
    )>,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_scroll: EventReader<MouseWheel>,
) {
    let mut zoom = 0.0;

    for event in mouse_scroll.read() {
        zoom += event.y;
    }

    let mut movement = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        movement.x += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        movement.x -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        movement.y += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        movement.y -= 1.0;
    }

    if !movement.is_zero() {
        movement = movement.normalize() * MOVE_SPEED * time.delta().as_secs_f32();
    }

    if keyboard.pressed(KeyCode::ShiftLeft) {
        movement *= SPEED_UP_FACTOR;
    }

    if keyboard.pressed(KeyCode::ControlLeft) {
        movement /= SPEED_UP_FACTOR;
    }

    let window_width = if let Some(window) = windows.iter().next() {
        window.width()
    } else {
        return;
    };

    if let Some((mut transform, mut camera, mut projection)) = cameras.iter_mut().next() {
        camera.view = (camera.view + camera.view * ZOOM_SPEED * zoom).round();
        transform.translation.x += movement.x * camera.view;
        transform.translation.y += movement.y * camera.view;
        projection.scale = camera.view / window_width;
    }
}
