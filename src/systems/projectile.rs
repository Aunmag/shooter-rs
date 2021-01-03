use crate::components::Collision;
use crate::components::Interpolation;
use crate::components::Projectile;
use crate::data::LAYER_PROJECTILE;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use crate::utils::math;
use amethyst::core::math::Point3;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::Read;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::Write;
use amethyst::ecs::Entities;
use amethyst::ecs::Entity;
use amethyst::renderer::debug_drawing::DebugLines;
use amethyst::renderer::palette::Srgba;

const FORCE_FACTOR: f32 = 1.0 / 4_000.0;

#[derive(SystemDesc)]
pub struct ProjectileSystem;

struct Obstacle {
    entity: Entity,
    distance_squared: f32,
}

impl<'a> System<'a> for ProjectileSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, Projectile>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Interpolation>,
        Write<'a, GameTaskResource>,
        Write<'a, DebugLines>,
    );

    fn run(
        &mut self,
        (
            entities,
            time,
            projectiles,
            collisions,
            transforms,
            interpolations,
            mut tasks,
            mut debug
        ): Self::SystemData,
    ) {
        for (entity, projectile) in (&entities, &projectiles).join() {
            let mut data = projectile.calc_data(time.absolute_time(), time.delta_seconds());
            let mut obstacle: Option<Obstacle> = None;

            for (entity, collision, transform, interpolation) in (
                &entities,
                &collisions,
                &transforms,
                (&interpolations).maybe(),
            )
                .join()
            {
                let mut obstacle_x = transform.translation().x;
                let mut obstacle_y = transform.translation().y;

                if let Some(interpolation) = interpolation {
                    obstacle_x += interpolation.offset_x;
                    obstacle_y += interpolation.offset_y;
                }

                if is_collision(
                    data.head.x,
                    data.head.y,
                    data.tail.x,
                    data.tail.y,
                    obstacle_x,
                    obstacle_y,
                    collision.radius,
                ) {
                    let distance_squared = math::distance_squared(
                        data.head.x,
                        data.head.y,
                        obstacle_x,
                        obstacle_y,
                    );

                    if obstacle.as_ref().map_or(true, |o| o.distance_squared < distance_squared) {
                        obstacle = Some(Obstacle {
                            entity,
                            distance_squared,
                        });
                    }
                }
            }

            if let Some(obstacle) = obstacle.as_ref() {
                let (sin, cos) = math::angle(
                    data.head.x,
                    data.head.y,
                    data.tail.x,
                    data.tail.y,
                ).sin_cos();

                let distance = obstacle.distance_squared.sqrt();

                data.head.x -= distance * cos;
                data.head.y -= distance * sin;

                tasks.push(GameTask::ProjectileHit {
                    entity: obstacle.entity,
                    force_x: data.velocity.x * FORCE_FACTOR,
                    force_y: data.velocity.y * FORCE_FACTOR,
                })
            }

            debug.draw_line(
                Point3::from([data.head.x, data.head.y, LAYER_PROJECTILE]),
                Point3::from([data.tail.x, data.tail.y, LAYER_PROJECTILE]),
                Srgba::new(1.0, 1.0, 0.0, 1.0),
            );

            if obstacle.is_some() || data.has_stopped() {
                if let Err(error) = entities.delete(entity) {
                    log::error!("Failed to delete a stopped bullet: {}", error);
                }
            }
        }
    }
}

fn is_collision(
    line_head_x: f32,
    line_head_y: f32,
    line_tail_x: f32,
    line_tail_y: f32,
    obstacle_x: f32,
    obstacle_y: f32,
    obstacle_radius: f32,
) -> bool {
    let x1 = obstacle_x - line_head_x;
    let y1 = obstacle_y - line_head_y;
    let x2 = obstacle_x - line_tail_x - x1;
    let y2 = obstacle_y - line_tail_y - y1;

    let a = x2 * x2 + y2 * y2;
    let b = (x1 * x2 + y1 * y2) * 2.0;
    let c = x1 * x1 + y1 * y1 - obstacle_radius * obstacle_radius;

    if b > 0.0 {
        return c < 0.0;
    } else if -b < 2.0 * a {
        return a * 4.0 * c - b * b < 0.0;
    } else {
        return a + b + c < 0.0;
    }
}
