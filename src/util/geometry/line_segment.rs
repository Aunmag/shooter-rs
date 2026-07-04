use crate::util::geometry::Geometry;
use bevy::math::Vec2;

pub type LineSegment = (Vec2, Vec2);

pub trait LineSegmentTrait {
    fn length_squared(&self) -> f32;
}

impl LineSegmentTrait for LineSegment {
    fn length_squared(&self) -> f32 {
        return self.0.distance_squared(self.1);
    }
}

impl Geometry for LineSegment {}
