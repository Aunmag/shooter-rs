use crate::{
    data::VIEW_DISTANCE,
    model::AppState,
    plugin::camera::MainCamera,
    util::ext::{AppExt, DurationExt, TransformExt},
};
use bevy::{
    app::{App, Plugin},
    ecs::{
        component::Component,
        event::EventReader,
        schedule::IntoSystemConfigs,
        system::{Query, Res},
    },
    input::mouse::MouseWheel,
    math::{Quat, Vec2, Vec3},
    prelude::{OrthographicProjection, Transform, With, Without},
    time::Time,
    window::{PrimaryWindow, Window},
};
use std::{
    f32::consts::FRAC_PI_2,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
    time::Duration,
};

const SHAKE_Y: f32 = 0.004;
const SHAKE_Z: f32 = 0.0006;
const SHAKE_R: f32 = 0.0007;

pub struct CameraTargetPlugin;

impl Plugin for CameraTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(
            AppState::Game,
            on_update.after(crate::plugin::collision::on_update),
        );
    }
}

#[derive(Default, Component)]
pub struct CameraTarget {
    pub sync_angle: Option<f32>,
    zoom: Zoom,
    shake_push: Shake<Vec2>,
    shake_spin: Shake<f32>,
    direction: f32,
}

impl CameraTarget {
    fn update(&mut self, zoom: f32, delta: f32) {
        self.zoom.add(zoom);
        self.zoom.update(delta);
        self.shake_spin.update(delta);
        self.shake_push.update(delta);
    }

    pub fn shake(&mut self, push: Vec2, spin: f32) {
        self.shake_push.add(push);
        self.shake_spin.add(spin);
    }

    pub const fn offset_y(&self) -> f32 {
        if self.sync_angle.is_some() {
            return 0.5;
        } else {
            return 0.0;
        }
    }
}

pub fn on_update(
    mut cameras: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    mut targets: Query<(&Transform, &mut CameraTarget), Without<MainCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut mouse_scroll: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    let mut zoom = 0.0;

    for event in mouse_scroll.read() {
        zoom += event.y;
    }

    let window_size = if let Some(window) = windows.iter().next() {
        Vec2::new(window.width(), window.height())
    } else {
        return;
    };

    if let Some((target_transform, mut target)) = targets.iter_mut().next() {
        target.update(zoom, delta);

        let scale = VIEW_DISTANCE
            / window_size.length()
            / (target.zoom.get() - target.shake_push.get().length() * SHAKE_Z * target.zoom.get());

        if let Some(offset_r) = target.sync_angle {
            target.direction = target_transform.direction() - FRAC_PI_2 - offset_r;
        };

        let rotation = Quat::from_rotation_z(target.direction + target.shake_spin.get() * SHAKE_R);

        let mut offset = Vec3::ZERO;
        offset.y += window_size.y / 2.0 * scale * target.offset_y();
        offset = rotation * offset;
        offset += target.shake_push.get().extend(0.0) * SHAKE_Y;

        if let Some((mut camera_transform, mut camera_projection)) = cameras.iter_mut().next() {
            camera_transform.translation.x = target_transform.translation.x + offset.x;
            camera_transform.translation.y = target_transform.translation.y + offset.y;
            camera_projection.scale = scale;
            camera_transform.rotation = rotation;
        }
    }
}

struct Zoom {
    value: f32,
    value_target: f32,
    speed: Duration,
}

impl Default for Zoom {
    fn default() -> Self {
        return Self {
            value: Self::MAX,
            value_target: Self::DEFAULT,
            speed: Self::SPEED_INITIAL,
        };
    }
}

impl Zoom {
    const SENSITIVITY: f32 = 0.2;

    const MIN: f32 = 1.0;
    const MAX: f32 = 5.0;
    const DEFAULT: f32 = 2.0;

    const SPEED_INITIAL: Duration = Duration::from_millis(5000);
    const SPEED_MANUAL: Duration = Duration::from_millis(125);

    fn update(&mut self, delta: f32) {
        self.value += (self.value_target - self.value) * self.speed.delta(delta);
        self.value = Self::clamp(self.value);
    }

    fn add(&mut self, zoom: f32) {
        if zoom == 0.0 {
            return;
        }

        self.value_target += zoom * self.value_target * Self::SENSITIVITY;
        self.value_target = Self::clamp(self.value_target);
        self.speed = Self::SPEED_MANUAL;
    }

    fn get(&self) -> f32 {
        return self.value;
    }

    fn clamp(value: f32) -> f32 {
        return value.clamp(Self::MIN, Self::MAX);
    }
}

#[derive(Default)]
struct Shake<T: Default> {
    value: T,
    value_target: T,
}

impl<T> Shake<T>
where
    T: Default
        + Copy
        + Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<f32, Output = T>,
{
    const SPEED_INCREASE: Duration = Duration::from_millis(35);
    const SPEED_DECREASE: Duration = Duration::from_millis(600);

    fn update(&mut self, delta: f32) {
        self.value_target -= self.value_target * Self::SPEED_DECREASE.delta(delta);
        self.value += (self.value_target - self.value) * Self::SPEED_INCREASE.delta(delta);
    }

    fn add(&mut self, shake: T) {
        self.value_target += shake;
    }

    fn get(&self) -> T {
        return self.value;
    }
}
