use crate::{
    model::AppState,
    plugin::{
        debug::debug_line,
        kinetics::{self, Kinetics},
    },
    util::{
        chunk::ChunkMap,
        ext::{AppExt, Vec2Ext},
    },
};
use bevy::{
    color::palettes::css::WHITE,
    ecs::{component::Component, system::Local},
    math::Vec2,
    prelude::{App, Entity, In, IntoSystem, IntoSystemConfigs, Plugin, Query, Transform, With},
};
use rand::Rng;
use std::f32::consts::TAU;

const DEBUG: bool = false;
const EXTRA_RESOLVE_DISTANCE: f32 = 0.0001;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(
            AppState::Game,
            before_update.pipe(on_update).after(kinetics::on_update),
        );
    }
}

#[derive(Component)]
pub struct Collision {
    pub radius: f32,
}

#[derive(Default)]
struct CollisionFindSystemData {
    previous_chunks: usize,
    previous_solutions: usize,
}

fn before_update(
    mut data: Local<CollisionFindSystemData>,
    query: Query<(Entity, &Collision, &Transform, &Kinetics)>,
) -> Vec<CollisionSolution> {
    let mut chunks = ChunkMap::new(data.previous_chunks);

    for (entity, collision, transform, kinetics) in query.iter() {
        let position = transform.translation.truncate();
        chunks.insert(position, (entity, collision, position, kinetics));
    }

    if DEBUG {
        for chunk_id in chunks.map.keys() {
            let v = |x: i32, y: i32| Vec2::new(x as f32, y as f32);
            let p = v(chunk_id.x, chunk_id.y);
            debug_line(p + v(0, 0), p + v(1, 0), WHITE);
            debug_line(p + v(1, 0), p + v(1, 1), WHITE);
            debug_line(p + v(1, 1), p + v(0, 1), WHITE);
            debug_line(p + v(0, 1), p + v(0, 0), WHITE);
        }
    }

    let mut solutions = Vec::with_capacity(data.previous_solutions);

    while let Some(((e1, c1, p1, k1), chunk_id)) = chunks.pop() {
        chunks.iter_neighbors(chunk_id, |(e2, c2, p2, k2)| {
            let distance = *p2 - p1;
            let distance_min = c1.radius + c2.radius;

            if distance.is_short(distance_min) {
                let angle = if distance.is_zero() {
                    Vec2::from_angle(rand::thread_rng().gen_range(0.0..TAU))
                } else {
                    distance.normalize()
                };

                let shift_distance =
                    (distance_min - distance.length()) / 2.0 + EXTRA_RESOLVE_DISTANCE;

                let shift = angle * shift_distance;
                let push = Kinetics::bounce(k1, k2, angle);
                append_solution(&mut solutions, e1, -shift, push);
                append_solution(&mut solutions, *e2, shift, -push);
            }
        });
    }

    data.previous_chunks = chunks.map.len();
    data.previous_solutions = solutions.len();

    return solutions;
}

pub fn on_update(
    In(mut solutions): In<Vec<CollisionSolution>>, // TODO: don't run if empty
    mut query: Query<(&mut Transform, &mut Kinetics), With<Collision>>,
) {
    for solution in solutions.drain(..) {
        if let Ok((mut transform, mut kinetics)) = query.get_mut(solution.entity) {
            kinetics.push(solution.push, 0.0, false);
            transform.translation.x += solution.shift.x;
            transform.translation.y += solution.shift.y;
        }
    }
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

pub struct CollisionSolution {
    entity: Entity,
    shift: Vec2,
    push: Vec2,
}
