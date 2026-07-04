use crate::util::geometry::Geometry;
use bevy::math::Vec2;

pub struct Line {
    pub origin: Vec2,
    pub direction: Vec2,
}

impl Line {
    /// NOTE: Line direction must be normalized
    #[expect(dead_code, reason = "may use it later")]
    pub fn new(origin: Vec2, direction: Vec2) -> Self {
        debug_assert!(direction.is_normalized(), "Direction must be normalized");
        return Self { origin, direction };
    }
}

impl Geometry for Line {}
