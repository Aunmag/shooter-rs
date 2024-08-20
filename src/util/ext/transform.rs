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

    fn calc_angle(degrees: f32) -> String {
        return format!(
            "{:.2}",
            Quat::from_rotation_z(degrees / 180.0 * PI).angle_z() / PI * 180.0,
        );
    }

    #[test]
    fn test_angle_z() {
        assert_eq!(calc_angle(0.0), "0.00");
        assert_eq!(calc_angle(45.0), "45.00");
        assert_eq!(calc_angle(90.0), "90.00");
        assert_eq!(calc_angle(180.0), "180.00");
        assert_eq!(calc_angle(360.0), "-360.00");
    }
}
