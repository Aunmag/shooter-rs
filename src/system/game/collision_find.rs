use crate::component::{Collision, CollisionSolution, Inertia};
use bevy::{
    ecs::system::Local,
    math::{Vec2, Vec3Swizzles},
    prelude::{Entity, Query, Transform},
};

#[derive(Default)]
pub struct CollisionFindSystemData {
    previous_solutions: usize,
}

pub fn collision_find(
    mut data: Local<CollisionFindSystemData>,
    query: Query<(Entity, &Collision, &Transform, &Inertia)>,
) -> Vec<CollisionSolution> {
    let mut solutions = Vec::with_capacity(data.previous_solutions);

    for (n, (e1, c1, t1, i1)) in query.iter().enumerate() {
        for (e2, c2, t2, i2) in query.iter().skip(n + 1) {
            if let Some(shift) =
                Collision::resolve(c1, c2, t1.translation.xy(), t2.translation.xy())
            {
                // TODO: maybe collision solutions would contain relative_angle
                let relative_angle = (t2.translation - t1.translation).xy().normalize();
                let push = Inertia::bounce(i1, i2, relative_angle);
                append_solution(&mut solutions, e1, shift, push);
                append_solution(&mut solutions, e2, -shift, -push);
            }
        }
    }

    data.previous_solutions = solutions.len();

    return solutions;
}

fn append_solution(
    solutions: &mut Vec<CollisionSolution>,
    entity: Entity,
    shift: Vec2,
    push: Vec2,
) {
    for solution in solutions.iter_mut() {
        if solution.entity == entity {
            solution.shift += shift;
            solution.push += push;
            return; // return if solution has found and modified
        }
    }

    // push a new one otherwise
    solutions.push(CollisionSolution {
        entity,
        shift,
        push,
    });
}
