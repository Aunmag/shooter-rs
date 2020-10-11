use crate::components::Player;
use crate::components::TransformSync;
use crate::utils::math;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::Read;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::WriteStorage;

const INTERPOLATION_FACTOR: f32 = 2.5 / 25.0; // TODO: Tweak, sync with input sync interval

#[derive(SystemDesc)]
pub struct InterpolationSystem;

impl<'a> System<'a> for InterpolationSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, TransformSync>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (time, players, transforms_sync, mut transforms): Self::SystemData) {
        let factor = time.delta_seconds() / INTERPOLATION_FACTOR;

        for (player, transform, transform_sync) in (
            (&players).maybe(),
            &mut transforms,
            &transforms_sync,
        ).join() {
            let translation = transform.translation_mut();
            translation.x = interpolate(translation.x, transform_sync.target_x, factor);
            translation.y = interpolate(translation.y, transform_sync.target_y, factor);

            if player.is_none() {
                transform.set_rotation_2d(interpolate_angle(
                    transform.euler_angles().2,
                    transform_sync.target_angle,
                    factor,
                ));
            }
        }
    }
}

fn interpolate(current: f32, target: f32, factor: f32) -> f32 {
    return current + (target - current) * factor;
}

fn interpolate_angle(current: f32, target: f32, factor: f32) -> f32 {
    return current + math::get_radians_difference(current, target) * factor;
}
