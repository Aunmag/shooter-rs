use crate::components::Actor;
use crate::components::ActorActions;
use amethyst::core::math::Vector3;
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
pub struct ActorSystem;

impl<'a> System<'a> for ActorSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Actor>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (time, actors, mut transforms): Self::SystemData) {
        let velocity = Actor::MOVEMENT_VELOCITY * time.delta_seconds();

        for (actor, transform) in (&actors, &mut transforms).join() {
            let mut movement_x = 0.0;
            let mut movement_y = 0.0;

            if actor.actions.contains(ActorActions::MOVEMENT_FORWARD) {
                movement_y += 1.0;
            }

            if actor.actions.contains(ActorActions::MOVEMENT_BACKWARD) {
                movement_y -= 1.0;
            }

            if actor.actions.contains(ActorActions::MOVEMENT_LEFTWARD) {
                movement_x -= 1.0;
            }

            if actor.actions.contains(ActorActions::MOVEMENT_RIGHTWARD) {
                movement_x += 1.0;
            }

            let (movement_x, movement_y) = normalize_movement_input(movement_x, movement_y);

            transform.rotate_2d(actor.rotation);

            let movement = transform.rotation()
                * Vector3::new(movement_x, movement_y, 0.0)
                * velocity;

            transform.prepend_translation(movement);
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