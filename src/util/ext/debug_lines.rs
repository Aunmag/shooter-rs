use crate::data::LAYER_TREE;
use bevy::prelude::{Color, Vec2};
use bevy_prototype_debug_lines::DebugLines;

pub trait DebugLinesExt {
    fn ln(&mut self, head: Vec2, tail: Vec2, color: Color);

    fn ln_d(&mut self, head: Vec2, direction: f32, color: Color);
}

impl DebugLinesExt for DebugLines {
    fn ln(&mut self, head: Vec2, tail: Vec2, color: Color) {
        let n = 3;

        for x in -n..n {
            for y in -n..n {
                let o = Vec2::new(x as f32 / 300.0, y as f32 / 300.0);
                self.line_colored(
                    (head + o).extend(LAYER_TREE),
                    (tail + o).extend(LAYER_TREE),
                    0.0,
                    color,
                );
            }
        }
    }

    fn ln_d(&mut self, head: Vec2, direction: f32, color: Color) {
        self.ln(head, head + Vec2::from_angle(direction), color);
    }
}
