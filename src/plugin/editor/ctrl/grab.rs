use bevy::{ecs::{component::Component, entity::Entity, query::With, system::{Query, Res}, world::World}, math::Vec2};
use crate::plugin::editor::{ctrl::select::Select, ctrl::cursor::Cursor, node::Node};

#[derive(Component)]
pub struct Grab {
    offset: Vec2,
}

impl Grab {
    pub fn apply_to_selected(world: &mut World) -> bool {
        let Some(cursor) = world.resource_ref::<Cursor>().p else {
            return false;
        };

        let mut inserts = Vec::new();

        for (entity, node) in world
            .query_filtered::<(Entity, &Node), With<Select>>()
            .iter(world)
        {
            inserts.push((entity, node.p - cursor));
        }

        if inserts.is_empty() {
            return false;
        }

        for (entity, offset) in inserts {
            world.entity_mut(entity).insert(Self {
                offset
            });
        }

        return true;
    }

    pub fn system(
        mut query: Query<(&mut Node, &Self)>,
        cursor: Res<Cursor>,
    ) {
        let Some(cursor) = cursor.p else {
            return;
        };

        for (mut node, grab) in query.iter_mut() {
            node.p = cursor + grab.offset;
        }
    }
}
