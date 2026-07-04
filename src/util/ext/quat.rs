use bevy::math::{Quat, Vec2};

pub trait QuatExt {
    fn angle_z(&self) -> f32;
    fn as_vec(&self) -> Vec2;
}

impl QuatExt for Quat {
    fn angle_z(&self) -> f32 {
        return f32::atan2(self.z, self.w) * 2.0;
    }

    fn as_vec(&self) -> Vec2 {
        let w = self.w;
        let z = self.z;
        let v = Vec2::new(w * w - z * z, w * z + z * w);
        debug_assert!(v.is_normalized());
        return v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_z() {
        let test = |a: f32| fmt(Quat::from_rotation_z(a.to_radians()).angle_z().to_degrees());
        assert_eq!(test(0.0), fmt(0.0));
        assert_eq!(test(45.0), fmt(45.0));
        assert_eq!(test(90.0), fmt(90.0));
        assert_eq!(test(180.0), fmt(180.0));
        assert_eq!(test(360.0), fmt(-360.0));
    }

    #[test]
    fn as_vec() {
        let test = |a: f32| {
            let r = Quat::from_rotation_z(a.to_radians())
                .as_vec()
                .to_angle()
                .to_degrees();

            return fmt(r);
        };

        assert_eq!(test(0.0), fmt(0.0));
        assert_eq!(test(45.0), fmt(45.0));
        assert_eq!(test(90.0), fmt(90.0));
        assert_eq!(test(180.0), fmt(-180.0));
        assert_eq!(test(360.0), fmt(0.0));
    }

    fn fmt(n: f32) -> String {
        return format!("{:.4}", n);
    }
}
