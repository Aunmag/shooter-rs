use crate::components::Projectile;
use crate::data::LAYER_PROJECTILE;
use amethyst::core::math::Point3;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::Read;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::Write;
use amethyst::ecs::Entities;
use amethyst::renderer::debug_drawing::DebugLines;
use amethyst::renderer::palette::Srgba;

#[derive(SystemDesc)]
pub struct ProjectileSystem;

impl<'a> System<'a> for ProjectileSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, Projectile>,
        Write<'a, DebugLines>,
    );

    fn run(&mut self, (entities, time, projectiles, mut debug): Self::SystemData) {
        for (entity, projectile) in (&entities, &projectiles).join() {
            let data = projectile.calc_data(time.absolute_time(), time.delta_seconds());

            debug.draw_line(
                Point3::from([data.head.x, data.head.y, LAYER_PROJECTILE]),
                Point3::from([data.tail.x, data.tail.y, LAYER_PROJECTILE]),
                Srgba::new(1.0, 1.0, 0.0, 1.0),
            );

            if data.has_stopped() {
                if let Err(error) = entities.delete(entity) {
                    log::error!("Failed to delete a stopped bullet: {}", error);
                }
            }
        }
    }
}
