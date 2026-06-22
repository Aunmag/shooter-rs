use crate::plugin::{debug::debug_circle, editor::control::Selection};
use bevy::{
    color::{
        palettes::css::{ORANGE, WHITE},
        Alpha,
    },
    ecs::{component::Component, query::Has, system::Query},
    math::Vec2,
};

#[derive(Component)]
pub struct Node {
    pub p: Vec2,
    pub r: f32,
}

impl Node {
    pub fn on_render(nodes: Query<(&Node, Has<Selection>)>) {
        for (node, selected) in nodes.iter() {
            debug_circle(node.p, node.r, WHITE.with_alpha(0.8));

            if selected {
                debug_circle(node.p, node.r + 0.05, ORANGE.with_alpha(1.0));
            }
        }
    }
}
