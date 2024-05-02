mod line;
mod line_segment;
mod point;

pub use self::{line::*, line_segment::*};

pub trait Geometry {}

pub trait GeometryProjection<T: Geometry> {
    fn project_on(self, g: &T) -> Self;
    fn project_on_clamped(self, g: &T) -> Self;
}

pub trait GeometryDistance<T: Geometry> {
    fn distance_squared(&self, g: &T) -> f32;
}
