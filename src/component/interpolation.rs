use crate::model::Position;
use crate::util::ext::DurationExt;
use crate::util::math;
use bevy::ecs::component::Component;
use std::time::Duration;

#[derive(Component)]
pub struct Interpolation {
    pub origin: Position,
    pub target: Position,
    pub start: Duration,
}

impl Interpolation {
    pub const fn new(position: Position, time: Duration) -> Self {
        return Self {
            origin: position,
            target: position,
            start: time,
        };
    }

    pub fn next(&mut self, position: Position, duration: Duration, time: Duration) {
        self.origin = self.get_interpolated_position(duration, time);
        self.target = position;
        self.start = time;
    }

    pub fn get_interpolated_position(&self, duration: Duration, time: Duration) -> Position {
        let progress = time.get_progress(self.start, self.get_end_time(duration));

        return Position::new(
            interpolate(self.origin.x, self.target.x, progress),
            interpolate(self.origin.y, self.target.y, progress),
            interpolate_angle(self.origin.direction, self.target.direction, progress),
        );
    }

    pub fn get_end_time(&self, duration: Duration) -> Duration {
        return self.start + duration;
    }
}

fn interpolate(source: f32, target: f32, progress: f32) -> f32 {
    return source + (target - source) * progress;
}

fn interpolate_angle(source: f32, target: f32, progress: f32) -> f32 {
    return source + math::angle_difference(source, target) * progress;
}
