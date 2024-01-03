use bevy::{
    math::{Quat, Vec2},
    prelude::Vec3Swizzles,
};

#[allow(clippy::wrong_self_convention)]
pub trait Vec2Ext {
    const FRONT: Vec2 = Vec2::new(1.0, 0.0);
    const BACK: Vec2 = Vec2::new(-1.0, 0.0);

    fn from_length(length: f32, angle: f32) -> Self;

    fn rotate_by_quat(self, quat: Quat) -> Self;

    fn angle(&self) -> f32;

    fn angle_to(self, target: Self) -> f32;

    fn distance_squared(self, target: Self) -> f32;

    fn is_zero(self) -> bool;

    fn is_close(self, target: Self, threshold: f32) -> bool;

    fn is_far(self, target: Self, threshold: f32) -> bool;

    fn is_long(self, threshold: f32) -> bool;

    fn is_short(self, threshold: f32) -> bool;
}

impl Vec2Ext for Vec2 {
    fn from_length(length: f32, angle: f32) -> Self {
        return Self::from_angle(angle) * length;
    }

    fn rotate_by_quat(self, quat: Quat) -> Self {
        return (quat * self.extend(0.0)).xy();
    }

    fn angle(&self) -> f32 {
        return f32::atan2(self.y, self.x);
    }

    fn angle_to(self, target: Self) -> f32 {
        return (target - self).angle();
    }

    fn distance_squared(self, target: Self) -> f32 {
        return (self - target).length_squared();
    }

    fn is_zero(self) -> bool {
        return self.x == 0.0 && self.y == 0.0;
    }

    fn is_close(self, target: Self, threshold: f32) -> bool {
        return self.distance_squared(target) < threshold * threshold;
    }

    fn is_far(self, target: Self, threshold: f32) -> bool {
        return !self.is_close(target, threshold);
    }

    fn is_long(self, threshold: f32) -> bool {
        return self.length_squared() > threshold * threshold;
    }

    fn is_short(self, threshold: f32) -> bool {
        return !self.is_long(threshold);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{math::normalize_radians, test::assert_radians_eq};
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

    #[test]
    fn test_from_length() {
        for length in [0.5, 1.0, 13.2] {
            for angle in [
                -TAU,
                -PI - FRAC_PI_2,
                -PI,
                -FRAC_PI_2,
                0.0,
                FRAC_PI_2,
                PI,
                PI + FRAC_PI_2,
                TAU,
            ] {
                let vec = Vec2::from_length(length, angle);
                assert_radians_eq!(vec.angle(), normalize_radians(angle));
                assert_eq!(vec.length(), length);
            }
        }
    }

    #[test]
    fn test_angle_to() {
        for c in [Vec2::ZERO, Vec2::new(1.0, 1.0), Vec2::new(-34.6, 44.2)] {
            for distance in [0.1, 2349.4] {
                let x = Vec2::new(distance, 0.0);
                let y = Vec2::new(0.0, distance);
                assert_eq!(c.angle_to(c + x), 0.0);
                assert_eq!(c.angle_to(c - x), PI);
                assert_eq!(c.angle_to(c + y), FRAC_PI_2);
                assert_eq!(c.angle_to(c - y), -FRAC_PI_2);
                assert_eq!(c.angle_to(c + x + y), FRAC_PI_4);
                assert_eq!(c.angle_to(c + x - y), -FRAC_PI_4);
            }
        }
    }
}
