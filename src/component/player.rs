use bevy::ecs::component::Component;
use std::time::Duration;

#[derive(Default, Component)]
pub struct Player {
    zoom: Zoom,
}

impl Player {
    pub fn update(&mut self, time: Duration, delta: f32) {
        self.zoom.update(time, delta);
    }

    pub fn add_zoom(&mut self, zoom: f32, time: Duration) {
        self.zoom.add(zoom, time);
    }

    pub fn get_zoom(&self) -> f32 {
        return self.zoom.get();
    }
}

struct Zoom {
    value: f32,
    value_target: f32,
    speed: f32,
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

    const SPEED_INITIAL: f32 = 0.2;
    const SPEED_MANUAL: f32 = 8.0;
    const SPEED_RESET: f32 = 1.25;

    const RESET_TIMEOUT: Duration = Duration::from_secs(4);

    pub fn update(&mut self, time: Duration, delta: f32) {
        self.value += (self.value_target - self.value) * delta * self.speed;
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
