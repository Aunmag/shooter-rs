use crate::components::Interpolation;
use crate::components::Player;
use crate::systems::net::transform_sync::INTERVAL as TRANSFORM_SYNC_INTERVAL;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::Read;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::WriteStorage;

const INTERPOLATION_FACTOR: f32 = 2.5; // TODO: Tweak

#[derive(SystemDesc)]
pub struct InterpolationSystem;

impl<'a> System<'a> for InterpolationSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Interpolation>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (time, players, mut interpolations, mut transforms): Self::SystemData) {
        // TODO: Use float constant in future
        let mut factor =
            time.delta_seconds() / (INTERPOLATION_FACTOR * TRANSFORM_SYNC_INTERVAL.as_secs_f32());

        // TODO: Use clamp method in future
        if factor > 1.0 {
            factor = 1.0;
        } else if factor < 0.0 {
            factor = 0.0;
        }

        let query = (&mut transforms, &mut interpolations, (&players).maybe()).join();

        for (transform, interpolation, player) in query {
            transform.prepend_translation_x(interpolation.offset_x * factor);
            transform.prepend_translation_y(interpolation.offset_y * factor);

            if player.is_none() {
                transform.rotate_2d(interpolation.offset_direction * factor);
            }

            let negative_factor = 1.0 - factor;
            interpolation.offset_x *= negative_factor;
            interpolation.offset_y *= negative_factor;
            interpolation.offset_direction *= negative_factor;
        }
    }
}
