use crate::components::Actor;
use crate::components::ActorActions;
use crate::components::RigidBody;
use crate::utils;
use amethyst::core::math::Vector3;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::Join;
use amethyst::ecs::Read;
use amethyst::ecs::ReadStorage;
use amethyst::ecs::System;
use amethyst::ecs::WriteStorage;

pub struct ActorSystem;

impl<'a> System<'a> for ActorSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Actor>,
        WriteStorage<'a, RigidBody>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (time, actors, mut bodies, mut transforms): Self::SystemData) {
        let velocity = Actor::MOVEMENT_VELOCITY * time.delta_seconds();

        for (actor, body, transform) in (&actors, &mut bodies, &mut transforms).join() {
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

            movement = transform.rotation() * normalize_movement(movement) * velocity;

            body.push(movement.x, movement.y, 0.0, false, true);
        }
    }
}

fn normalize_movement(mut movement: Vector3<f32>) -> Vector3<f32> {
    let length_squared = utils::math::length_squared(movement.x, movement.y);

    if length_squared > 1.0 {
        let length = length_squared.sqrt();
        movement.x /= length;
        movement.y /= length;
    }

    return movement;
}
