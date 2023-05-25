use crate::{
    model::TransformLite,
    util::{ext::DurationExt, math},
};
use bevy::ecs::component::Component;
use std::time::Duration;

#[derive(Component)]
pub struct Interpolation {
    pub origin: TransformLite,
    pub target: TransformLite,
    pub start: Duration,
}

impl Interpolation {
    pub const fn new(transform: TransformLite, time: Duration) -> Self {
        return Self {
            origin: transform,
            target: transform,
            start: time,
        };
    }

    pub fn next(&mut self, transform: TransformLite, duration: Duration, time: Duration) {
        self.origin = self.get_interpolated_transform(duration, time);
        self.target = transform;
        self.start = time;
    }

    pub fn get_interpolated_transform(&self, duration: Duration, time: Duration) -> TransformLite {
        let progress = time.get_progress(self.start, self.get_end_time(duration));

        return TransformLite::new(
            math::interpolate(
                self.origin.translation.x,
                self.target.translation.x,
                progress,
            ),
            math::interpolate(
                self.origin.translation.y,
                self.target.translation.y,
                progress,
            ),
            interpolate_angle(self.origin.direction, self.target.direction, progress),
        );
    }

    pub fn get_end_time(&self, duration: Duration) -> Duration {
        return self.start + duration;
    }
}

fn interpolate_angle(source: f32, target: f32, progress: f32) -> f32 {
    return source + math::angle_difference(source, target) * progress;
}
