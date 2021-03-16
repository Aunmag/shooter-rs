use crate::data::POSITION_UPDATE_INTERVAL;
use crate::utils::math;
use crate::utils::DurationExt;
use crate::utils::Position;
use amethyst::ecs::Component;
use amethyst::ecs::VecStorage;
use std::time::Duration;

pub struct Interpolation {
    origin: Position,
    target: Position,
    start: Duration,
}

impl Interpolation {
    pub fn new(position: Position, now: Duration) -> Self {
        return Self {
            origin: position,
            target: position,
            start: now,
        };
    }

    pub fn next(&mut self, position: Position, now: Duration) {
        self.origin = self.get_interpolated_position(now);
        self.target = position;
        self.start = now;
    }

    pub fn shift(&mut self, x: f32, y: f32) {
        self.origin.x += x;
        self.origin.y += y;
        self.target.x += x;
        self.target.y += y;
    }

    pub fn get_interpolated_position(&self, time: Duration) -> Position {
        let progress = time.get_progress(self.start, self.start + POSITION_UPDATE_INTERVAL);

        return Position::new(
            interpolate(self.origin.x, self.target.x, progress),
            interpolate(self.origin.y, self.target.y, progress),
            interpolate_angle(self.origin.direction, self.target.direction, progress),
        );
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
