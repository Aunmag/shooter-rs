use crate::util;
use bevy::prelude::EulerRot;
use bevy::prelude::Transform;

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
