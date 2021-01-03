use crate::components::Collision;
use crate::components::Interpolation;
use amethyst::core::math::Point2;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::WriteStorage;
use amethyst::ecs::Entities;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(SystemDesc)]
pub struct CollisionSystem {
    previous_entities_count: usize,
}

struct Solution {
    push_x: f32,
    push_y: f32,
}

impl CollisionSystem {
    pub fn new() -> Self {
        return Self {
            previous_entities_count: 10,
        };
    }
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Interpolation>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (e, c, i, mut t): Self::SystemData) {
        let mut checked = HashSet::with_capacity(self.previous_entities_count * 2);
        let mut solutions = HashMap::with_capacity(self.previous_entities_count);

        for (e1, t1, i1, c1) in (&e, &t, (&i).maybe(), &c).join() {
            let p1 = to_point(t1, i1);

            for (e2, t2, i2, c2) in (&e, &t, (&i).maybe(), &c).join() {
                if e1.id() == e2.id() || checked.contains(&e2.id()) {
                    continue;
                }

                let p2 = to_point(t2, i2);

                if let Some(solution) = Collision::resolve(c1, c2, p1, p2) {
                    append_solution(&mut solutions, e1.id(), solution.x, solution.y);
                    append_solution(&mut solutions, e2.id(), -solution.x, -solution.y);
                }
            }

            checked.insert(e1.id());
        }

        if !solutions.is_empty() {
            for (entity, transform) in (&e, &mut t).join() {
                if let Some(solution) = solutions.remove(&entity.id()) {
                    let translation = transform.translation_mut();
                    translation.x += solution.push_x;
                    translation.y += solution.push_y;
                }

                if solutions.is_empty() {
                    break;
                }
            }
        }

        self.previous_entities_count = checked.len();
    }
}

fn to_point(transform: &Transform, interpolation: Option<&Interpolation>) -> Point2<f32> {
    let mut x = transform.translation().x;
    let mut y = transform.translation().y;

    if let Some(interpolation) = interpolation {
        x += interpolation.offset_x;
        y += interpolation.offset_y;
    }

    return Point2::from([x, y]);
}

fn append_solution(
    solutions: &mut HashMap<u32, Solution>,
    entity_id: u32,
    push_x: f32,
    push_y: f32,
) {
    solutions
        .entry(entity_id)
        .and_modify(|s| {
            s.push_x += push_x;
            s.push_y += push_y;
        })
        .or_insert_with(|| Solution { push_x, push_y });
}
