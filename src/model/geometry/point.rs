use crate::model::geometry::{
    Geometry, GeometryDistance, GeometryProjection, Line, LineSegment, LineSegmentTrait,
};
use bevy::math::Vec2;

impl Geometry for Vec2 {}

impl GeometryProjection<Line> for Vec2 {
    /// NOTE: Line direction must be normalized
    fn project_on(self, l: &Line) -> Vec2 {
        debug_assert!(l.direction.is_normalized(), "Direction must be normalized");
        return l.origin + l.direction * l.direction.dot(self - l.origin);
    }
}

impl GeometryProjection<LineSegment> for Vec2 {
    fn project_on(self, l: &LineSegment) -> Vec2 {
        let p = self;
        let v = l.0;
        let u = l.1;
        let l_length = l.length_squared();

        if l_length == 0.0 {
            return u;
        }

        let t = Vec2::dot(p - v, u - v) / l_length;

        if t < 0.0 {
            return v;
        }

        if t > 1.0 {
            return u;
        }

        return v + t * (u - v);
    }
}

impl GeometryDistance<Line> for Vec2 {
    fn distance_squared(&self, l: &Line) -> f32 {
        return Vec2::distance_squared(*self, self.project_on(l));
    }
}

impl GeometryDistance<LineSegment> for Vec2 {
    fn distance_squared(&self, l: &LineSegment) -> f32 {
        return Vec2::distance_squared(*self, self.project_on(l));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_on_line_segment() {
        let line = (Vec2::new(37.0, 84.0), Vec2::new(63.0, 24.0));
        assert_eq!(Vec2::new(38.0, 93.0).project_on(&line), line.0);
        assert_eq!(Vec2::new(53.0, 11.0).project_on(&line), line.1);
        assert_eq!(
            Vec2::new(92.0, 82.0).project_on(&line).round(),
            Vec2::new(46.0, 62.0),
        );

        let line = (Vec2::new(1.0, 2.0), Vec2::new(1.0, 2.0));
        assert_eq!(Vec2::new(4.0, 77.0).project_on(&line), line.0);
    }
}
