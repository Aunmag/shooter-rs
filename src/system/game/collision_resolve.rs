use crate::component::{Collision, CollisionSolution, Inertia};
use bevy::prelude::{Entity, In, Query, Transform, With};

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
