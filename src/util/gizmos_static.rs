use bevy::{
    gizmos::gizmos::Gizmos,
    prelude::{Color, Vec2},
};
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref GIZMOS: GizmosStatic = GizmosStatic::default();
}

#[derive(Default)]
pub struct GizmosStatic {
    lines: Mutex<Vec<(Vec2, Vec2, Color)>>,
}

#[allow(clippy::unwrap_used)]
impl GizmosStatic {
    pub fn render(&self, gizmos: &mut Gizmos) {
        for (head, tail, color) in self.lines.lock().unwrap().drain(..) {
            gizmos.line_2d(head, tail, color);
        }
    }

    pub fn ln(&self, head: Vec2, tail: Vec2, color: Color) {
        self.lines.lock().unwrap().push((head, tail, color));
    }
}
