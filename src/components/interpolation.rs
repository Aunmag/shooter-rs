use crate::data::POSITION_UPDATE_INTERVAL;
use crate::utils::math;
use crate::utils::DurationExt;
use crate::utils::Position;
use amethyst::core::math::Vector2;
use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;
use std::time::Duration;

pub struct Interpolation {
    origin: Position,
    target: Position,
    start: Duration,
}

impl Interpolation {
    pub const fn new(position: Position, now: Duration) -> Self {
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
        let progress = time.get_progress(self.start, self.get_end_time());

        return Position::new(
            interpolate(self.origin.x, self.target.x, progress),
            interpolate(self.origin.y, self.target.y, progress),
            interpolate_angle(self.origin.direction, self.target.direction, progress),
        );
    }

    pub fn get_approximate_velocity(&self, time: Duration) -> Vector2<f32> {
        if time < self.get_end_time() {
            return Vector2::new(self.target.x - self.origin.x, self.target.y - self.origin.y)
                / POSITION_UPDATE_INTERVAL.as_secs_f32();
        } else {
            return Vector2::new(0.0, 0.0);
        }
    }

    pub fn get_end_time(&self) -> Duration {
        return self.start + POSITION_UPDATE_INTERVAL;
    }
}

impl Component for Interpolation {
    type Storage = DenseVecStorage<Self>;
}

fn interpolate(source: f32, target: f32, progress: f32) -> f32 {
    return source + (target - source) * progress;
}

fn interpolate_angle(source: f32, target: f32, progress: f32) -> f32 {
    return source + math::angle_difference(source, target) * progress;
}
