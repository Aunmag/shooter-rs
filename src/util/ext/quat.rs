use bevy::math::{Quat, Vec2};

pub trait QuatExt {
    fn perp(self) -> Self;
    fn angle_z(&self) -> f32;
    fn as_vec(&self) -> Vec2;
}

impl QuatExt for Quat {
    /// Returns perpendicular quat (rotated by 90 degrees)
    fn perp(self) -> Self {
        return self
            * Self::from_xyzw(
                0.0,
                0.0,
                (1.0 / std::f64::consts::SQRT_2) as f32,
                (1.0 / std::f64::consts::SQRT_2) as f32,
            );
    }

    /// Faster than `Quat::to_euler`
    fn angle_z(&self) -> f32 {
        return f32::atan2(self.z, self.w) * 2.0;
    }

    fn as_vec(&self) -> Vec2 {
        let w = self.w;
        let z = self.z;
        let v = Vec2::new(w * w - z * z, w * z * 2.0);
        // debug_assert!(v.is_normalized()); // TODO: check
        return v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::math::normalize_radians;
    use rand::RngExt;
    use std::f32::consts::{FRAC_PI_2, TAU};

    const TESTS: u32 = 10_000;

    #[test]
    fn perp() {
        for _ in 0..TESTS {
            let a = random_angle();
            assert_eq!(
                f(Quat::from_rotation_z(a).perp().angle_z()),
                f(Quat::from_rotation_z(a + FRAC_PI_2).angle_z()),
            );
        }
    }

    #[test]
    fn angle_z() {
        for _ in 0..TESTS {
            let a = random_angle();
            assert_eq!(f(Quat::from_rotation_z(a).angle_z()), f(a));
        }
    }

    #[test]
    fn as_vec() {
        for _ in 0..TESTS {
            let a = random_angle();
            assert_eq!(f(Quat::from_rotation_z(a).as_vec().to_angle()), f(a));
        }
    }

    fn f(r: f32) -> String {
        return format!("{:.2}", normalize_radians(r)); // TODO: increase precision
    }

    fn random_angle() -> f32 {
        return rand::rng().random_range(-TAU..TAU);
    }
}
