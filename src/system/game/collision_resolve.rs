use crate::component::Collision;
use crate::component::CollisionSolution;
use crate::component::Inertia;
use bevy::prelude::Entity;
use bevy::prelude::In;
use bevy::prelude::Query;
use bevy::prelude::Transform;
use bevy::prelude::With;

pub fn collision_resolve(
    In(mut solutions): In<Vec<CollisionSolution>>,
    mut query: Query<(Entity, &mut Transform, &mut Inertia), With<Collision>>,
) {
    if solutions.is_empty() {
        return;
    }

    // TODO: try to invert
    for (entity, mut transform, mut inertia) in query.iter_mut() {
        for (i, solution) in solutions.iter().enumerate() {
            if entity.index() == solution.entity_index {
                inertia.push(solution.push, 0.0, true, false, false);
                transform.translation.x += solution.shift.x;
                transform.translation.y += solution.shift.y;
                solutions.swap_remove(i);

                if solutions.is_empty() {
                    return;
                } else {
                    break;
                }
            }
        }
    }
}
