use crate::model::geometry::{Geometry, GeometryDistance};
use bevy::math::Vec2;

pub struct Line {
    pub origin: Vec2,
    pub direction: Vec2,
}

impl Line {
    /// NOTE: Line direction must be normalized
    #[allow(dead_code)] // maybe I'll use it later
    pub fn new(origin: Vec2, direction: Vec2) -> Self {
        debug_assert!(direction.is_normalized(), "Direction must be normalized");
        return Self { origin, direction };
    }
}

impl Geometry for Line {}

impl GeometryDistance<Vec2> for Line {
    fn distance_squared(&self, p: &Vec2) -> f32 {
        return p.distance_squared(self);
    }
}
