use crate::{data::PIXELS_PER_METER, util::ext::TransformExt};
use bevy::{
    math::{Vec2, Vec3Swizzles},
    prelude::{Quat, Transform, Vec3},
};

// TODO: no copy
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
            scale: Vec3::splat(1.0 / PIXELS_PER_METER),
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
