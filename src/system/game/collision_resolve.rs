use crate::component::{Collision, CollisionSolution, Inertia};
use bevy::prelude::{In, Query, Transform, With};

pub fn collision_resolve(
    In(mut solutions): In<Vec<CollisionSolution>>, // TODO: don't run if empty
    mut query: Query<(&mut Transform, &mut Inertia), With<Collision>>,
) {
    for solution in solutions.drain(..) {
        if let Ok((mut transform, mut inertia)) = query.get_mut(solution.entity) {
            inertia.push(solution.push, 0.0, false);
            transform.translation.x += solution.shift.x;
            transform.translation.y += solution.shift.y;
        }
    }
}
