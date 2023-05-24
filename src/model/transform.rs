use crate::data::PIXELS_PER_METER;
use crate::util;
use crate::util::ext::TransformExt;
use bevy::math::Vec2;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Quat;
use bevy::prelude::Transform;
use bevy::prelude::Vec3;
use serde::Deserialize;
use serde::Serialize;

// TODO: no copy
#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

impl From<TransformLiteU8> for TransformLite {
    fn from(transform: TransformLiteU8) -> Self {
        return Self {
            translation: transform.translation,
            direction: transform.direction_f32(),
        };
    }
}

// TODO: no copy
#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransformLiteU8 {
    pub translation: Vec2,
    pub direction: u8,
}

impl TransformLiteU8 {
    pub const fn new(x: f32, y: f32, direction: u8) -> Self {
        return Self {
            translation: Vec2::new(x, y),
            direction,
        };
    }

    pub fn as_transform(self, z: f32) -> Transform {
        return Transform {
            translation: self.translation.extend(z),
            rotation: Quat::from_rotation_z(self.direction_f32()),
            scale: Vec3::splat(1.0 / PIXELS_PER_METER),
        };
    }

    pub fn direction_f32(&self) -> f32 {
        return util::math::decompress_radians(self.direction);
    }
}

impl From<&Transform> for TransformLiteU8 {
    fn from(transform: &Transform) -> Self {
        return Self {
            translation: transform.translation.xy(),
            direction: transform.direction_u8(),
        };
    }
}

impl From<TransformLite> for TransformLiteU8 {
    fn from(transform: TransformLite) -> Self {
        return Self {
            translation: transform.translation,
            direction: util::math::compress_radians(transform.direction),
        };
    }
}
