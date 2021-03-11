use crate::data::POSITION_UPDATE_INTERVAL;
use crate::utils::math;
use crate::utils::DurationExt;
use amethyst::ecs::Component;
use amethyst::ecs::VecStorage;
use std::time::Duration;

pub struct Interpolation {
    origin: InterpolationPosition,
    target: InterpolationPosition,
    start: Duration,
}

pub struct InterpolationPosition {
    pub x: f32,
    pub y: f32,
    pub direction: f32,
}

impl Interpolation {
    pub fn new(x: f32, y: f32, direction: f32, now: Duration) -> Self {
        return Self {
            origin: InterpolationPosition::new(x, y, direction),
            target: InterpolationPosition::new(x, y, direction),
            start: now,
        };
    }

    pub fn next(&mut self, x: f32, y: f32, direction: f32, now: Duration) {
        self.origin = self.get_interpolated_position(now);
        self.target = InterpolationPosition::new(x, y, direction);
        self.start = now;
    }

    pub fn shift(&mut self, x: f32, y: f32) {
        self.origin.x += x;
        self.origin.y += y;
        self.target.x += x;
        self.target.y += y;
    }

    pub fn get_interpolated_position(&self, time: Duration) -> InterpolationPosition {
        let progress = time.get_progress(self.start, self.start + POSITION_UPDATE_INTERVAL);

        return InterpolationPosition::new(
            interpolate(self.origin.x, self.target.x, progress),
            interpolate(self.origin.y, self.target.y, progress),
            interpolate_angle(self.origin.direction, self.target.direction, progress),
        );
    }
}

impl InterpolationPosition {
    pub fn new(x: f32, y: f32, direction: f32) -> Self {
        return Self { x, y, direction };
    }
}

impl Component for Interpolation {
    type Storage = VecStorage<Self>;
}

fn interpolate(source: f32, target: f32, progress: f32) -> f32 {
    return source + (target - source) * progress;
}

fn interpolate_angle(source: f32, target: f32, progress: f32) -> f32 {
    return source + math::angle_difference(source, target) * progress;
}
