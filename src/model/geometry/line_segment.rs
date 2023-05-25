use crate::model::geometry::{Geometry, GeometryDistance};
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

impl GeometryDistance<Vec2> for LineSegment {
    fn distance_squared(&self, p: &Vec2) -> f32 {
        return p.distance_squared(self);
    }
}
