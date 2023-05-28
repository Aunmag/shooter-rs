use crate::util::ext::DurationExt;
use bevy::ecs::component::Component;
use std::time::Duration;

#[derive(Default, Component)]
pub struct Player {
    zoom: Zoom,
    shake: Shake,
    shake_abs: Shake,
    extra_rotation: f32,
}

impl Player {
    pub const EXTRA_ROTATION_MULTIPLAYER: f32 = 0.1;
    pub const EXTRA_ROTATION_MAX: f32 = 0.15;

    pub fn update(&mut self, time: Duration, delta: f32) {
        self.zoom.update(time, delta);
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

    pub fn add_zoom(&mut self, zoom: f32, time: Duration) {
        self.zoom.add(zoom, time);
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

struct Zoom {
    value: f32,
    value_target: f32,
    speed: Duration,
    reset_time: Duration,
}

impl Default for Zoom {
    fn default() -> Self {
        return Self {
            value: Self::INITIAL,
            value_target: Self::DEFAULT,
            speed: Self::SPEED_INITIAL,
            reset_time: Duration::ZERO,
        };
    }
}

impl Zoom {
    const SENSITIVITY: f32 = 0.2;

    const MIN: f32 = 1.0;
    const MAX: f32 = 8.0;
    const DEFAULT: f32 = 1.7;
    const INITIAL: f32 = 5.0;

    const SPEED_INITIAL: Duration = Duration::from_millis(5000);
    const SPEED_MANUAL: Duration = Duration::from_millis(125);
    const SPEED_RESET: Duration = Duration::from_millis(800);

    const RESET_TIMEOUT: Duration = Duration::from_secs(4);

    pub fn update(&mut self, time: Duration, delta: f32) {
        self.value += (self.value_target - self.value) * delta * self.speed.rate();
        self.value = Self::clamp(self.value);

        if !self.reset_time.is_zero() && time > self.reset_time {
            self.value_target = Self::DEFAULT;
            self.speed = Self::SPEED_RESET;
            self.reset_time = Duration::ZERO;
        }
    }

    pub fn add(&mut self, zoom: f32, time: Duration) {
        if zoom == 0.0 {
            return;
        }

        self.value_target += zoom * self.value_target * Self::SENSITIVITY;
        self.value_target = Self::clamp(self.value_target);
        self.speed = Self::SPEED_MANUAL;

        // reset only when distant
        if self.value_target > Self::DEFAULT {
            self.reset_time = Duration::ZERO;
        } else {
            self.reset_time = time + Self::RESET_TIMEOUT;
        }
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
    const SENSITIVITY: f32 = 0.000001;
    const SPEED_INCREASE: Duration = Duration::from_millis(35);
    const SPEED_DECREASE: Duration = Duration::from_millis(600);

    pub fn update(&mut self, delta: f32) {
        self.value_target -= self.value_target * delta * Self::SPEED_DECREASE.rate();
        self.value += (self.value_target - self.value) * delta * Self::SPEED_INCREASE.rate();
    }

    pub fn add(&mut self, shake: f32) {
        self.value_target += shake * Self::SENSITIVITY;
    }

    fn get(&self) -> f32 {
        return self.value;
    }
}
