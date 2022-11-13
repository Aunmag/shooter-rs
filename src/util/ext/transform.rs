use bevy::prelude::EulerRot;
use bevy::prelude::Transform;

pub trait TransformExt {
    fn direction(&self) -> f32;
}

impl TransformExt for Transform {
    fn direction(&self) -> f32 {
        return self.rotation.to_euler(EulerRot::ZXY).0;
    }
}
