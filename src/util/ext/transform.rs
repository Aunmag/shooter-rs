use crate::util;
use bevy::prelude::{EulerRot, Transform};

pub trait TransformExt {
    fn direction(&self) -> f32;

    fn direction_u8(&self) -> u8 {
        return util::math::compress_radians(self.direction());
    }
}

impl TransformExt for Transform {
    fn direction(&self) -> f32 {
        return self.rotation.to_euler(EulerRot::ZXY).0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test::assert_radians_eq;
    use std::f32::consts::FRAC_PI_4;

    #[test]
    fn test_direction() {
        let mut transform = Transform::IDENTITY;
        assert_radians_eq!(transform.direction(), 0.0);
        transform.rotate_local_z(FRAC_PI_4);
        assert_radians_eq!(transform.direction(), FRAC_PI_4);
        transform.rotate_local_z(-FRAC_PI_4);
        assert_radians_eq!(transform.direction(), 0.0);
    }
}
