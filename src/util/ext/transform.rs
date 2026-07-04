use bevy::math::Quat;

pub trait QuatExt {
    fn angle_z(&self) -> f32;
}

impl QuatExt for Quat {
    fn angle_z(&self) -> f32 {
        return f32::atan2(self.z, self.w) * 2.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn angle_z() {
        let test = |a: f32| {
            let converted = Quat::from_rotation_z(a / 180.0 * PI).angle_z() / PI * 180.0;
            return format!("{:.2}", converted);
        };

        assert_eq!(test(0.0), "0.00");
        assert_eq!(test(45.0), "45.00");
        assert_eq!(test(90.0), "90.00");
        assert_eq!(test(180.0), "180.00");
        assert_eq!(test(360.0), "-360.00");
    }
}
