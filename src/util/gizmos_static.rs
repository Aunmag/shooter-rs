use super::ext::Vec2Ext;
use bevy::{
    gizmos::gizmos::Gizmos,
    prelude::{Color, Vec2},
};
use std::{f32::consts::TAU, sync::Mutex};

lazy_static::lazy_static! {
    pub static ref GIZMOS: GizmosStatic = GizmosStatic::default();
}

const CIRCLE_FACES: u8 = 24;
const CIRCLE_STEP: f32 = TAU / CIRCLE_FACES as f32;

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

    pub fn circle(&self, center: Vec2, radius: f32, color: Color) {
        let mut lines = self.lines.lock().unwrap();

        for i in 0..CIRCLE_FACES {
            let p0 = center + Vec2::from_length(radius, CIRCLE_STEP * i as f32);
            let p1 = center + Vec2::from_length(radius, CIRCLE_STEP * (i + 1) as f32);
            lines.push((p0, p1, color));
        }
    }
}
