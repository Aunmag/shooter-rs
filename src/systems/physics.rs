use crate::components::Collision;
use crate::components::Interpolation;
use crate::components::Own;
use crate::components::RigidBody;
use amethyst::core::math::Vector2;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::Read;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::WriteStorage;
use amethyst::ecs::Entities;

#[derive(SystemDesc)]
pub struct PhysicsSystem {
    is_optimal: bool,
    previous_collisions_count: usize,
}

struct Solution {
    entity_id: u32,
    shift: Vector2<f32>,
    push: Vector2<f32>,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        return Self {
            is_optimal: true,
            previous_collisions_count: 0,
        };
    }
}

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Own>,
        WriteStorage<'a, Interpolation>,
        WriteStorage<'a, RigidBody>,
        WriteStorage<'a, Transform>,
    );

    #[allow(clippy::many_single_char_names)]
    fn run(&mut self, (e, time, c, o, mut i, mut b, mut t): Self::SystemData) {
        let delta = time.delta_seconds();
        let mut last_checked_entity_id = 0;
        let mut solutions = Vec::with_capacity(self.previous_collisions_count * 2);

        self.previous_collisions_count = 0;

        for (e1, t1, c1, b1) in (&e, &t, &c, (&b).maybe()).join() {
            let p1 = predict_position(t1, b1, delta);

            for (e2, t2, c2, b2, _own) in (&e, &t, &c, (&b).maybe(), &o).join() {
                if e1.id() == e2.id() || e2.id() <= last_checked_entity_id {
                    continue;
                }

                let p2 = predict_position(t2, b2, delta);

                if let Some(shift) = Collision::resolve(c1, c2, p1, p2) {
                    let mut push_1 = Vector2::new(0.0, 0.0);
                    let mut push_2 = Vector2::new(0.0, 0.0);

                    if let (Some(b1), Some(b2)) = (b1, b2) {
                        // TODO: Maybe collision solutions would contain relative_angle
                        let relative_angle = (t2.translation() - t1.translation()).xy().normalize();
                        let push = RigidBody::bounce(b1, b2, relative_angle);
                        push_1 += &push;
                        push_2 -= &push;
                    }

                    append_solution(&mut solutions, e1.id(), shift, push_1);
                    append_solution(&mut solutions, e2.id(), -shift, push_2);
                    self.previous_collisions_count += 1;
                }
            }

            if self.is_optimal {
                if e1.id() < last_checked_entity_id {
                    last_checked_entity_id = 0;
                    self.is_optimal = false;
                    log::warn!("The system may not work optimally since entities aren't sorted");
                } else {
                    last_checked_entity_id = e1.id();
                }
            }
        }

        for (entity, transform, interpolation, mut body, own, _collision) in (
            &e,
            &mut t,
            (&mut i).maybe(),
            (&mut b).maybe(),
            (&o).maybe(),
            (&c),
        )
            .join()
        {
            let previous_position = transform.translation().xy();
            let mut applied_solution = None;

            for (i, solution) in solutions.iter().enumerate() {
                if entity.id() == solution.entity_id {
                    let translation = transform.translation_mut();
                    translation.x += solution.shift.x;
                    translation.y += solution.shift.y;

                    if let (Some(body), Some(..)) = (body.as_mut(), own) {
                        body.push(solution.push.x, solution.push.y, 0.0, true, false);
                    }

                    applied_solution.replace(i);
                    break;
                }
            }

            if let Some(applied_solution) = applied_solution {
                solutions.swap_remove(applied_solution);
            }

            if let (Some(body), Some(..)) = (body.as_mut(), own) {
                let translation = transform.translation_mut();
                translation.x += body.velocity.x * delta;
                translation.y += body.velocity.y * delta;

                transform.rotate_2d(body.spinning * delta);

                body.velocity *= 1.0 - delta * body.drag;
                body.spinning *= 1.0 - delta * body.drag_angular;
            }

            if let Some(interpolation) = interpolation {
                let shift = transform.translation().xy() - previous_position;
                interpolation.shift(shift.x, shift.y);
            }
        }
    }
}

fn predict_position(transform: &Transform, body: Option<&RigidBody>, delta: f32) -> Vector2<f32> {
    let mut predicted = transform.translation().xy();

    if let Some(body) = body {
        predicted += body.velocity * delta;
    }

    return predicted;
}

fn append_solution(
    solutions: &mut Vec<Solution>,
    entity_id: u32,
    shift: Vector2<f32>,
    push: Vector2<f32>,
) {
    for solution in solutions.iter_mut() {
        if solution.entity_id == entity_id {
            solution.shift += &shift;
            solution.push += &push;
            return; // Return if solution has found and modified
        }
    }

    // Push a new one otherwise
    solutions.push(Solution {
        entity_id,
        shift,
        push,
    });
}
