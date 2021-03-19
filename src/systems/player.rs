use crate::components::Actor;
use crate::components::ActorActions;
use crate::components::Player;
use crate::input::ActionBinding;
use crate::input::AxisBinding;
use crate::input::CustomBindingTypes;
use crate::resources::MouseInput;
use amethyst::ecs::Join;
use amethyst::ecs::Read;
use amethyst::ecs::ReadStorage;
use amethyst::ecs::System;
use amethyst::ecs::Write;
use amethyst::ecs::WriteStorage;
use amethyst::input::InputHandler;
use std::f32::consts::TAU;

const ROTATION_SENSITIVITY: f32 = 0.003;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        Read<'a, InputHandler<CustomBindingTypes>>,
        ReadStorage<'a, Player>,
        Write<'a, MouseInput>,
        WriteStorage<'a, Actor>,
    );

    fn run(&mut self, (input, players, mut input_mouse, mut actors): Self::SystemData) {
        let rotation = (input_mouse.delta_x * ROTATION_SENSITIVITY) % TAU;

        for (actor, _) in (&mut actors, &players).join() {
            actor.rotation = rotation;

            apply_movement_input(
                &mut actor.actions,
                ActorActions::MOVEMENT_FORWARD,
                ActorActions::MOVEMENT_BACKWARD,
                input
                    .axis_value(&AxisBinding::MovementForward)
                    .unwrap_or(0.0),
            );

            apply_movement_input(
                &mut actor.actions,
                ActorActions::MOVEMENT_RIGHTWARD,
                ActorActions::MOVEMENT_LEFTWARD,
                input.axis_value(&AxisBinding::MovementAside).unwrap_or(0.0),
            );

            actor.actions.set(
                ActorActions::ATTACK,
                input
                    .action_is_down(&ActionBinding::Attack)
                    .unwrap_or(false),
            );
        }

        input_mouse.delta_x = 0.0;
        input_mouse.delta_y = 0.0;
    }
}

fn apply_movement_input(actions: &mut ActorActions, a: ActorActions, b: ActorActions, ratio: f32) {
    if ratio > 0.0 {
        *actions |= a;
        *actions -= b;
    } else if ratio < 0.0 {
        *actions -= a;
        *actions |= b;
    } else {
        *actions -= a;
        *actions -= b;
    }
}
