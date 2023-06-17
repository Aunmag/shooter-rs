use crate::util::ext::DebugLinesExt;
use bevy::prelude::{Color, Vec2};
use bevy_prototype_debug_lines::DebugLines;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref DEBUG_LINES: DebugLinesStatic = DebugLinesStatic::default();
}

#[derive(Default)]
pub struct DebugLinesStatic {
    lines: Mutex<Vec<(Vec2, Vec2, Color)>>,
}

#[allow(clippy::unwrap_used)]
impl DebugLinesStatic {
    pub fn render(&self, debug_lines: &mut DebugLines) {
        for (head, tail, color) in self.lines.lock().unwrap().drain(..) {
            debug_lines.ln(head, tail, color);
        }
    }

    pub fn ln(&self, head: Vec2, tail: Vec2, color: Color) {
        self.lines.lock().unwrap().push((head, tail, color));
    }
}
