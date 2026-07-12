use bevy::{color::{Alpha, palettes::css::ORANGE}, ecs::{component::Component, entity::Entity, query::With, system::Query, world::World}, hierarchy::DespawnRecursiveExt};

use crate::plugin::{debug::debug_circle, editor::node::Node};

#[derive(Component)]
pub struct Select;

impl Select {
    pub fn insert(world: &mut World, entity: Entity, add: bool) {
        if !add {
            Self::remove_all(world);
        }

        world.entity_mut(entity).insert(Self);
    }

    pub fn remove_all(world: &mut World) -> bool {
        return crate::plugin::editor::util::remove_component_from_all::<Self>(world);
    }

    pub fn delete_selected(world: &mut World) {
        for entity in world
            .query_filtered::<Entity, With<Self>>()
            .iter(world)
            .collect::<Vec<_>>()
        {
            world.entity_mut(entity).despawn_recursive();
        }
    }

    pub fn system(query: Query<&Node, With<Self>>) {
        for node in query.iter() {
            debug_circle(node.p, node.r + 0.05, ORANGE.with_alpha(1.0));
        }
    }
}
