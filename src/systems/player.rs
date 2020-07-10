use crate::components::actor::Actor;
use crate::components::player::Player;
use crate::utils;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::Read;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::WriteStorage;
use amethyst::input::InputHandler;
use amethyst::input::StringBindings;

const MOVEMENT_VELOCITY: f32 = 50.0;
const ROTATION_SENSITIVITY: f32 = 0.01;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        ReadStorage<'a, Actor>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (actors, players, mut transforms, time, input): Self::SystemData) {
        for (_, _, transform) in (&actors, &players, &mut transforms).join() {
            let mut move_x = 0.0;
            let mut move_y = 0.0;

            if input.action_is_down("move_forward").unwrap_or(false) {
                move_y += MOVEMENT_VELOCITY;
            }

            if input.action_is_down("move_backwards").unwrap_or(false) {
                move_y -= MOVEMENT_VELOCITY;
            }

            if input.action_is_down("move_left").unwrap_or(false) {
                move_x -= MOVEMENT_VELOCITY;
            }

            if input.action_is_down("move_right").unwrap_or(false) {
                move_x += MOVEMENT_VELOCITY;
            }

            transform.rotate_2d(utils::input::take_mouse_delta() as f32 * ROTATION_SENSITIVITY);

            // TODO: Optimize, avoid calculating cos and sin
            let delta = time.delta_seconds();
            let angle = transform.euler_angles().2;
            let angle_perpendicular = angle - utils::math::PI_0_5;
            transform.prepend_translation_x(move_x * angle_perpendicular.cos() * delta);
            transform.prepend_translation_y(move_x * angle_perpendicular.sin() * delta);
            transform.prepend_translation_x(move_y * angle.cos() * delta);
            transform.prepend_translation_y(move_y * angle.sin() * delta);
        }
    }
}
