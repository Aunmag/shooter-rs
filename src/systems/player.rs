use crate::components::Actor;
use crate::components::Player;
use crate::input;
use crate::input::AxisBinding;
use crate::input::CustomBindingTypes;
use amethyst::core::math::Vector3;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::Read;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::WriteStorage;
use amethyst::input::InputHandler;

const ROTATION_SENSITIVITY: f32 = 0.01;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        Read<'a, InputHandler<CustomBindingTypes>>,
        Read<'a, Time>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (input, time, mut players, mut transforms): Self::SystemData) {
        for (player, transform) in (&mut players, &mut transforms).join() {
            let rotation = input::take_mouse_delta() as f32 * ROTATION_SENSITIVITY;

            transform.rotate_2d(rotation);

            let (movement_x, movement_y) = normalize_movement_input(
                input.axis_value(&AxisBinding::MoveAside).unwrap_or(0.0),
                input.axis_value(&AxisBinding::MoveForward).unwrap_or(0.0),
            );

            let movement = transform.rotation()
                * Vector3::new(movement_x, movement_y, 0.0)
                * time.delta_seconds();

            transform.prepend_translation(movement * Actor::MOVEMENT_VELOCITY);

            player
                .accumulated_input
                .prepend(movement.x, movement.y, rotation);
        }
    }
}

fn normalize_movement_input(x: f32, y: f32) -> (f32, f32) {
    let movement_squared = x * x + y * y;

    if movement_squared > 1.0 {
        let movement = movement_squared.sqrt();
        return (1.0 * x / movement, 1.0 * y / movement);
    } else {
        return (x, y);
    }
}
