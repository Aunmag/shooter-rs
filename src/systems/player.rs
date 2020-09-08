use crate::components::Actor;
use crate::components::Player;
use crate::input;
use crate::input::AxisBinding;
use crate::input::CustomBindingTypes;
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

const MOVEMENT_VELOCITY: f32 = 50.0;
const ROTATION_SENSITIVITY: f32 = 0.01;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        Read<'a, InputHandler<CustomBindingTypes>>,
        Read<'a, Time>,
        ReadStorage<'a, Actor>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (input, time, actors, players, mut transforms): Self::SystemData) {
        for (_, _, transform) in (&actors, &players, &mut transforms).join() {
            let move_forward = input.axis_value(&AxisBinding::MoveForward).unwrap_or(0.0)
                * MOVEMENT_VELOCITY
                * time.delta_seconds();

            let move_aside = input.axis_value(&AxisBinding::MoveAside).unwrap_or(0.0)
                * MOVEMENT_VELOCITY
                * time.delta_seconds();

            transform.rotate_2d(input::take_mouse_delta() as f32 * ROTATION_SENSITIVITY);
            transform.move_up(move_forward);
            transform.move_right(move_aside);
        }
    }
}
