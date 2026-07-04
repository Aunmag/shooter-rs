use crate::util::ext::QuatExt;
use bevy::{math::Vec2, prelude::Transform};

#[derive(Default, Clone, Copy, Debug)]
pub struct Transform2D {
    pub position: Vec2,
    pub rotation: f32,
}

impl From<&Transform> for Transform2D {
    fn from(transform: &Transform) -> Self {
        return Self {
            position: transform.translation.truncate(),
            rotation: transform.rotation.angle_z(),
        };
    }
}
