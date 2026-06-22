use crate::{
    plugin::{debug::debug_line, editor::node::Node},
    util::ext::Vec2Ext,
};
use bevy::{
    color::{palettes::css::WHITE, Alpha},
    ecs::{component::Component, entity::Entity, system::Query},
    hierarchy::DespawnRecursiveExt,
    math::Vec2,
    prelude::Commands,
};

#[derive(Component)]
pub struct Edge {
    pub a: Entity,
    pub b: Entity,
}

impl Edge {
    pub fn on_render(edges: Query<(Entity, &Edge)>, nodes: Query<&Node>, mut commands: Commands) {
        for (entity, edge) in edges.iter() {
            // TODO: simplify
            if let (Ok(a), Ok(b)) = (nodes.get(edge.a), nodes.get(edge.b)) {
                let ar = a.r;
                let br = b.r;
                let a = a.p;
                let b = b.p;

                let angle = a.angle_to(b) + std::f32::consts::FRAC_PI_2;
                let foo = Vec2::from_length(ar, angle);
                let bar = Vec2::from_length(br, angle);
                debug_line(a + foo, b + bar, WHITE.with_alpha(0.4));
                debug_line(a - foo, b - bar, WHITE.with_alpha(0.4));
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
