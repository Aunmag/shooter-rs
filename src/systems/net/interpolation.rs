use crate::components::Interpolation;
use crate::components::Player;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::Read;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::WriteStorage;

#[derive(SystemDesc)]
pub struct InterpolationSystem;

impl<'a> System<'a> for InterpolationSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Interpolation>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (time, interpolations, players, mut transforms): Self::SystemData) {
        let now = time.absolute_time();
        let query = (&interpolations, &mut transforms, (&players).maybe()).join();

        for (interpolation, transform, player) in query {
            let interpolated = interpolation.get_interpolated_position(now);
            transform.translation_mut().x = interpolated.x;
            transform.translation_mut().y = interpolated.y;

            if player.is_none() {
                transform.set_rotation_2d(interpolated.direction);
            }
        }
    }
}
