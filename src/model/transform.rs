use crate::{data::TRANSFORM_SCALE, util::ext::TransformExt};
use bevy::{
    math::{Vec2, Vec3Swizzles},
    prelude::{Quat, Transform},
};

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct TransformLite {
    pub translation: Vec2,
    pub direction: f32,
}

impl TransformLite {
    pub const fn new(x: f32, y: f32, direction: f32) -> Self {
        return Self {
            translation: Vec2::new(x, y),
            direction,
        };
    }

    pub fn as_transform(self, z: f32) -> Transform {
        return Transform {
            translation: self.translation.extend(z),
            rotation: Quat::from_rotation_z(self.direction),
            scale: TRANSFORM_SCALE,
        };
    }
}

impl From<&Transform> for TransformLite {
    fn from(transform: &Transform) -> Self {
        return Self {
            translation: transform.translation.xy(),
            direction: transform.direction(),
        };
    }
}
