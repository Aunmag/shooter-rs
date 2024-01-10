use crate::util::ext::DurationExt;
use bevy::ecs::{component::Component, entity::Entity};
use std::time::Duration;

#[derive(Component)]
pub struct Player {
    pub is_controllable: bool,
    pub crosshair: PlayerCrosshair,
    zoom: Zoom,
    shake: Shake,
    shake_abs: Shake,
    extra_rotation: f32,
}

impl Player {
    pub const EXTRA_ROTATION_MULTIPLAYER: f32 = 0.1;
    pub const EXTRA_ROTATION_MAX: f32 = 0.11;

    pub fn new(is_controllable: bool, crosshair: Entity) -> Self {
        return Self {
            is_controllable,
            crosshair: PlayerCrosshair::new(crosshair),
            zoom: Zoom::default(),
            shake: Shake::default(),
            shake_abs: Shake::default(),
            extra_rotation: 0.0,
        };
    }

    pub fn update(&mut self, delta: f32) {
        self.zoom.update(delta);
        self.shake.update(delta);
        self.shake_abs.update(delta);
    }

    pub fn add_extra_rotation(&mut self, value: f32) -> f32 {
        let previous = self.extra_rotation;
        let limit = Self::EXTRA_ROTATION_MAX;
        self.extra_rotation = (self.extra_rotation + value).clamp(-limit, limit);
        let added = self.extra_rotation - previous;
        return added;
    }

    pub fn add_zoom(&mut self, zoom: f32) {
        self.zoom.add(zoom);
    }

    pub fn shake(&mut self, shake: f32) {
        self.shake.add(shake);
        self.shake_abs.add(shake.abs());
    }

    pub fn get_zoom(&self) -> f32 {
        return self.zoom.get();
    }

    pub fn get_shake(&self) -> f32 {
        return self.shake.get();
    }

    pub fn get_shake_abs(&self) -> f32 {
        return self.shake_abs.get();
    }

    pub fn get_extra_rotation(&self) -> f32 {
        return self.extra_rotation;
    }
}

pub struct PlayerCrosshair {
    pub entity: Entity,
    pub distance: f32,
}

impl PlayerCrosshair {
    pub fn new(entity: Entity) -> Self {
        return Self {
            entity,
            distance: 1.0,
        };
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

    pub fn update(&mut self, delta: f32) {
        self.value += (self.value_target - self.value) * self.speed.delta(delta);
        self.value = Self::clamp(self.value);
    }

    pub fn add(&mut self, zoom: f32) {
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
struct Shake {
    value: f32,
    value_target: f32,
}

impl Shake {
    const SENSITIVITY: f32 = 0.001;
    const SPEED_INCREASE: Duration = Duration::from_millis(35);
    const SPEED_DECREASE: Duration = Duration::from_millis(600);

    pub fn update(&mut self, delta: f32) {
        self.value_target -= self.value_target * Self::SPEED_DECREASE.delta(delta);
        self.value += (self.value_target - self.value) * Self::SPEED_INCREASE.delta(delta);
    }

    pub fn add(&mut self, shake: f32) {
        self.value_target += shake * Self::SENSITIVITY;
    }

    fn get(&self) -> f32 {
        return self.value;
    }
}
