use crate::component::{Collision, CollisionSolution, Inertia};
use bevy::{
    ecs::system::Resource,
    math::{Vec2, Vec3Swizzles},
    prelude::{Entity, Query, ResMut, Transform},
};

#[derive(Default, Resource)]
pub struct CollisionSystemData {
    previous_solutions: usize,
}

#[allow(clippy::many_single_char_names)]
pub fn collision_find(
    query: Query<(Entity, &Collision, &Transform, &Inertia)>,
    mut data: ResMut<CollisionSystemData>,
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
                append_solution(&mut solutions, e1.index(), shift, push);
                append_solution(&mut solutions, e2.index(), -shift, -push);
            }
        }
    }

    data.previous_solutions = solutions.len();

    return solutions;
}

fn append_solution(
    solutions: &mut Vec<CollisionSolution>,
    entity_index: u32,
    shift: Vec2,
    push: Vec2,
) {
    for solution in solutions.iter_mut() {
        if solution.entity_index == entity_index {
            solution.shift += shift;
            solution.push += push;
            return; // return if solution has found and modified
        }
    }

    // push a new one otherwise
    solutions.push(CollisionSolution {
        entity_index,
        shift,
        push,
    });
}
