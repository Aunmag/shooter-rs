use crate::components::Actor;
use crate::components::ActorActions;
use crate::components::Interpolation;
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
        WriteStorage<'a, Interpolation>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (time, actors, mut interpolations, mut transforms): Self::SystemData) {
        let velocity = Actor::MOVEMENT_VELOCITY * time.delta_seconds();

        for (actor, transform, interpolation) in (
            &actors,
            &mut transforms,
            (&mut interpolations).maybe(),
        )
            .join()
        {
            transform.rotate_2d(actor.rotation);

            if actor.actions.is_empty() {
                continue;
            }

            let mut movement = Vector3::new(0.0, 0.0, 0.0);

            if actor.actions.contains(ActorActions::MOVEMENT_FORWARD) {
                movement.y += 1.0;
            }

            if actor.actions.contains(ActorActions::MOVEMENT_BACKWARD) {
                movement.y -= 1.0;
            }

            if actor.actions.contains(ActorActions::MOVEMENT_LEFTWARD) {
                movement.x -= 1.0;
            }

            if actor.actions.contains(ActorActions::MOVEMENT_RIGHTWARD) {
                movement.x += 1.0;
            }

            let previous_position = transform.translation().xy();

            normalize_movement(&mut movement);
            transform.prepend_translation(transform.rotation() * movement * velocity);

            if let Some(interpolation) = interpolation {
                let shift = transform.translation().xy() - previous_position;
                interpolation.shift(shift.x, shift.y);
            }
        }
    }
}

fn normalize_movement(movement: &mut Vector3<f32>) {
    let length_squared = movement.x * movement.x + movement.y * movement.y;

    if length_squared > 1.0 {
        let length = length_squared.sqrt();
        movement.x /= length;
        movement.y /= length;
    }
}
