use crate::plugin::{debug::debug_circle};
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
    pub fn on_render(nodes: Query<&Node>) {
        for node in nodes.iter() {
            debug_circle(node.p, node.r, WHITE.with_alpha(0.8));
        }
    }
}
