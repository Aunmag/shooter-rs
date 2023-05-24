use bevy::math::Vec2;

pub trait Vec2Ext {
    fn from_length(length: f32, angle: f32) -> Self;

    fn angle(&self) -> f32;

    fn angle_to(self, other: Self) -> f32;

    fn is_longer_than(&self, value: f32) -> bool;

    fn is_shorter_than(&self, value: f32) -> bool {
        return !self.is_longer_than(value);
    }
}

impl Vec2Ext for Vec2 {
    fn from_length(length: f32, angle: f32) -> Self {
        return Self::from_angle(angle) * length;
    }

    fn angle(&self) -> f32 {
        return f32::atan2(self.y, self.x);
    }

    fn angle_to(self, other: Self) -> f32 {
        return (other - self).angle();
    }

    fn is_longer_than(&self, value: f32) -> bool {
        return self.length_squared() > value * value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::math::normalize_radians;
    use crate::util::test::assert_radians_eq;
    use std::f32::consts::FRAC_PI_2;
    use std::f32::consts::FRAC_PI_4;
    use std::f32::consts::PI;
    use std::f32::consts::TAU;

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
