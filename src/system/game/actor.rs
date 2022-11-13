use crate::component::Actor;
use crate::component::ActorActions;
use crate::component::Inertia;
use bevy::ecs::system::Query;
use bevy::math::Quat;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Res;
use bevy::prelude::Time;
use bevy::prelude::Vec3;
use bevy::transform::components::Transform;

pub fn actor(mut query: Query<(&Actor, &mut Inertia, &mut Transform)>, time: Res<Time>) {
    for (actor, mut inertia, mut transform) in query.iter_mut() {
        if actor.rotation != 0.0 {
            transform.rotate(Quat::from_rotation_z(actor.rotation));
        }

        if actor.actions.is_empty() {
            continue;
        }

        let mut movement = Vec3::new(0.0, 0.0, 0.0);

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

        movement = transform.rotation
            * normalize_movement(movement)
            * actor.config.movement_velocity
            * time.delta_seconds();

        inertia.push(movement.xy(), 0.0, false, true);
    }
}

fn normalize_movement(mut movement: Vec3) -> Vec3 {
    let length_squared = movement.length_squared();

    if length_squared > 1.0 {
        let length = length_squared.sqrt();
        movement.x /= length;
        movement.y /= length;
    }

    return movement;
}
