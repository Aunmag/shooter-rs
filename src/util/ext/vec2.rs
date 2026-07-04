use bevy::math::{Quat, Vec2};

#[expect(clippy::wrong_self_convention, reason = "it is a copy type")]
pub trait Vec2Ext {
    const FRONT: Vec2 = Vec2::new(1.0, 0.0);
    const BACK: Vec2 = Vec2::new(-1.0, 0.0);

    fn from_length(length: f32, angle: f32) -> Self;
    fn rotate_by_quat(self, quat: Quat) -> Self;
    fn angle_to(self, target: Self) -> f32;
    fn distance_squared(self, target: Self) -> f32;
    fn is_zero(self) -> bool;
    fn is_close(self, target: Self, threshold: f32) -> bool;
    fn is_far(self, target: Self, threshold: f32) -> bool;
    fn is_long(self, threshold: f32) -> bool;
    fn is_short(self, threshold: f32) -> bool;
    fn as_quat(self) -> Quat;
}

impl Vec2Ext for Vec2 {
    fn from_length(length: f32, angle: f32) -> Self {
        return Self::from_angle(angle) * length;
    }

    // TODO: unit-tests
    fn rotate_by_quat(self, q: Quat) -> Self {
        let v = self;
        let rw = q.w * q.w - q.z * q.z;
        let rz = 2.0 * q.w * q.z;
        return Vec2::new(v.x * rw - v.y * rz, v.x * rz + v.y * rw);
    }

    fn angle_to(self, target: Self) -> f32 {
        return (target - self).to_angle();
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
        return self.length_squared() >= threshold * threshold;
    }

    fn is_short(self, threshold: f32) -> bool {
        return !self.is_long(threshold);
    }

    fn as_quat(self) -> Quat {
        return Quat::from_rotation_z(self.to_angle());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{ext::QuatExt, test::assert_radians_eq};

    const ANGLES: [f32; 9] = [
        -std::f32::consts::TAU,       // -360°
        -std::f32::consts::PI,        // -180°
        -std::f32::consts::FRAC_PI_2, // -90°
        -std::f32::consts::FRAC_PI_4, // -45°
        0.0,                          // 0°
        std::f32::consts::FRAC_PI_4,  // 45°
        std::f32::consts::FRAC_PI_2,  // 90°
        std::f32::consts::PI,         // 180°
        std::f32::consts::TAU,        // 360°
    ];

    #[test]
    fn from_length() {
        for a in ANGLES {
            for l in [0.5, 1.0, 13.2] {
                let v = Vec2::from_length(l, a);
                assert_eq!(fmt(v.length()), fmt(l));
                assert_radians_eq!(v.to_angle(), a);
            }
        }
    }

    #[test]
    fn angle_to() {
        for a in ANGLES {
            let v1 = Vec2::new(12.3, 45.6);
            let v2 = v1 + Vec2::from_length(3.3, a);
            assert_radians_eq!(v1.angle_to(v2), a);
        }
    }

    #[test]
    fn as_quat() {
        for a in ANGLES {
            assert_radians_eq!(Vec2::from_angle(a).as_quat().angle_z(), a);
        }
    }

    fn fmt(n: f32) -> String {
        return format!("{:.5}", n);
    }
}
