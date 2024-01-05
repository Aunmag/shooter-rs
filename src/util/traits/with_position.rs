use crate::util::ext::Vec2Ext;
use bevy::{
    math::{Vec2, Vec3Swizzles},
    transform::components::Transform,
};

pub trait WithPosition {
    fn position(&self) -> Vec2;

    fn distance_squared<T: WithPosition>(&self, target: &T) -> f32 {
        return Vec2Ext::distance_squared(self.position(), target.position());
    }

    fn is_close<T: WithPosition>(&self, target: &T, threshold: f32) -> bool {
        return Vec2Ext::is_close(self.position(), target.position(), threshold);
    }

    fn is_far<T: WithPosition>(&self, target: &T, threshold: f32) -> bool {
        return Vec2Ext::is_far(self.position(), target.position(), threshold);
    }

    fn angle_to<T: WithPosition>(&self, target: &T) -> f32 {
        return Vec2Ext::angle_to(self.position(), target.position());
    }
}

impl WithPosition for Vec2 {
    fn position(&self) -> Vec2 {
        return *self;
    }
}

impl WithPosition for Transform {
    fn position(&self) -> Vec2 {
        return self.translation.xy();
    }
}
