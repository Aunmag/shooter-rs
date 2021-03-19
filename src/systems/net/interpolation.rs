use crate::components::Interpolation;
use crate::components::Player;
use crate::components::RigidBody;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::Join;
use amethyst::ecs::Read;
use amethyst::ecs::ReadStorage;
use amethyst::ecs::System;
use amethyst::ecs::WriteStorage;

pub struct InterpolationSystem;

impl<'a> System<'a> for InterpolationSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Interpolation>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, RigidBody>,
        WriteStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (time, interpolations, players, mut bodies, mut transforms): Self::SystemData,
    ) {
        let now = time.absolute_time();

        for (interpolation, transform, body, player) in (
            &interpolations,
            &mut transforms,
            (&mut bodies).maybe(),
            (&players).maybe(),
        )
            .join()
        {
            let interpolated = interpolation.get_interpolated_position(now);
            transform.translation_mut().x = interpolated.x;
            transform.translation_mut().y = interpolated.y;

            if player.is_none() {
                transform.set_rotation_2d(interpolated.direction);

                if let Some(body) = body {
                    body.velocity = interpolation.get_approximate_velocity(now);
                }
            }
        }
    }
}
