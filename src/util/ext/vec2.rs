use bevy::math::Vec2;
use std::f32::consts::FRAC_PI_2;

pub trait Vec2Ext {
    fn atan2(self) -> f32;

    fn atan2_to(self, other: Self) -> f32;

    fn direction(self: Self) -> f32
    where
        Self: Sized,
    {
        return self.atan2() + FRAC_PI_2;
    }

    fn is_longer_than(&self, value: f32) -> bool;

    fn is_shorter_than(&self, value: f32) -> bool {
        return !self.is_longer_than(value);
    }
}

impl Vec2Ext for Vec2 {
    fn atan2(self) -> f32 {
        return self.y.atan2(self.x);
    }

    fn atan2_to(self, other: Self) -> f32 {
        // TODO: compare with Vec2::angle
        return (self - other).atan2();
    }

    fn is_longer_than(&self, value: f32) -> bool {
        return self.length_squared() > value * value;
    }
}
