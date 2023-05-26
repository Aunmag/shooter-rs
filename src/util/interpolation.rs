use crate::util::ext::DurationExt;
use bevy::prelude::Vec2;
use std::time::Duration;

pub struct Interpolation {
    source: (Vec2, Duration),
    target: (Vec2, Duration),
}

impl Interpolation {
    pub fn new(value: Vec2, time: Duration) -> Self {
        return Self {
            source: (value, time),
            target: (value, time),
        };
    }

    pub fn add(&mut self, value: Vec2, time: Duration) {
        self.source = self.target;
        self.target = (value, time);
    }

    pub fn interpolate(&self, time: Duration) -> Vec2 {
        return self.source.0 + self.difference() * self.progress(time);
    }

    pub fn difference(&self) -> Vec2 {
        return self.target.0 - self.source.0;
    }

    pub fn progress(&self, time: Duration) -> f32 {
        return time.get_progress(self.target.1, self.target.1 + self.interval());
    }

    pub fn interval(&self) -> Duration {
        return self.target.1.saturating_sub(self.source.1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(x: f32, y: f32) -> Vec2 {
        return Vec2::new(x, y);
    }

    fn d(s: u64) -> Duration {
        return Duration::from_secs(s);
    }

    #[test]
    fn test_interpolate_new() {
        let i = Interpolation::new(v(1.0, 2.0), d(2));
        assert_eq!(v(1.0, 2.0), i.interpolate(d(0)));
        assert_eq!(v(1.0, 2.0), i.interpolate(d(1)));
        assert_eq!(v(1.0, 2.0), i.interpolate(d(2)));
        assert_eq!(v(1.0, 2.0), i.interpolate(d(3)));
    }

    #[test]
    fn test_interpolate_complex() {
        let mut i = Interpolation::new(v(1.0, 2.0), d(2));
        i.add(v(-1.0, 0.0), d(4));
        assert_eq!(v(1.0, 2.0), i.interpolate(d(0)));
        assert_eq!(v(1.0, 2.0), i.interpolate(d(1)));
        assert_eq!(v(1.0, 2.0), i.interpolate(d(2)));
        assert_eq!(v(1.0, 2.0), i.interpolate(d(3)));
        assert_eq!(v(1.0, 2.0), i.interpolate(d(4)));
        assert_eq!(v(0.0, 1.0), i.interpolate(d(5)));
        assert_eq!(v(-1.0, 0.0), i.interpolate(d(6)));
        assert_eq!(v(-1.0, 0.0), i.interpolate(d(7)));
    }
}
