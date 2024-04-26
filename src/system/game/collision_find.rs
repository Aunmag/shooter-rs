use crate::{
    component::{Collision, CollisionSolution, Inertia},
    plugin::debug::debug_line,
    util::ext::HashMapExt,
};
use bevy::{
    ecs::system::Local,
    math::{Vec2, Vec3Swizzles},
    prelude::{Entity, Query, Transform},
    render::color::Color,
    utils::hashbrown::HashMap,
};

const DEBUG: bool = false;

#[derive(Default)]
pub struct CollisionFindSystemData {
    previous_chunks: usize,
    previous_solutions: usize,
}

pub fn collision_find(
    mut data: Local<CollisionFindSystemData>,
    query: Query<(Entity, &Collision, &Transform, &Inertia)>,
) -> Vec<CollisionSolution> {
    let mut chunks = HashMap::<ChunkId, Vec<Entity>>::with_capacity(data.previous_chunks);
    let offsets = [-1, 0, 1];

    for (entity, _, transform, _) in query.iter() {
        chunks
            .entry(ChunkId::from(transform.translation.truncate()))
            .or_insert_with(Vec::new)
            .push(entity);
    }

    if DEBUG {
        for chunk_id in chunks.keys() {
            let v = |x: i32, y: i32| Vec2::new(x as f32, y as f32);
            let p = v(chunk_id.x, chunk_id.y);
            debug_line(p + v(0, 0), p + v(1, 0), Color::WHITE);
            debug_line(p + v(1, 0), p + v(1, 1), Color::WHITE);
            debug_line(p + v(1, 1), p + v(0, 1), Color::WHITE);
            debug_line(p + v(0, 1), p + v(0, 0), Color::WHITE);
        }
    }

    let mut solutions = Vec::with_capacity(data.previous_solutions);

    while let Some((chunk_id, mut chunk)) = chunks.pop() {
        while let Some(e1) = chunk.pop() {
            let Ok((e1, c1, t1, i1)) = query.get(e1) else {
                continue;
            };

            for offset_x in &offsets {
                for offset_y in &offsets {
                    let chunk = if *offset_x == 0 && *offset_y == 0 {
                        &chunk // check in same chunk
                    } else if let Some(chunk) = chunks.get(&chunk_id.plus(*offset_x, *offset_y)) {
                        chunk // check in neighboring chunk
                    } else {
                        continue;
                    };

                    for e2 in chunk.iter() {
                        let Ok((e2, c2, t2, i2)) = query.get(*e2) else {
                            continue;
                        };

                        if let Some(shift) = Collision::resolve(
                            c1,
                            c2,
                            t1.translation.truncate(),
                            t2.translation.truncate(),
                        ) {
                            // TODO: maybe collision solutions would contain relative_angle
                            let relative_angle = (t2.translation - t1.translation).xy().normalize();
                            let push = Inertia::bounce(i1, i2, relative_angle);
                            append_solution(&mut solutions, e1, shift, push);
                            append_solution(&mut solutions, e2, -shift, -push);
                        }
                    }
                }
            }
        }
    }

    data.previous_chunks = chunks.len();
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ChunkId {
    x: i32,
    y: i32,
}

impl ChunkId {
    fn plus(self, x: i32, y: i32) -> Self {
        return Self {
            x: self.x + x,
            y: self.y + y,
        };
    }
}

impl From<Vec2> for ChunkId {
    fn from(position: Vec2) -> Self {
        return Self {
            x: position.x.floor() as i32,
            y: position.y.floor() as i32,
        };
    }
}
