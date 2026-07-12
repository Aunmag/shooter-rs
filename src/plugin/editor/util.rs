use bevy::ecs::{component::Component, entity::Entity, query::With, world::World};

pub fn remove_component_from_all<T: Component>(world: &mut World) -> bool {
    let mut removed = false;

    for entity in world
        .query_filtered::<Entity, With<T>>()
        .iter(world)
        .collect::<Vec<_>>()
    {
        world.entity_mut(entity).remove::<T>();
        removed = true;
    }

    return removed;
}
