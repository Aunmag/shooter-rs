use crate::component::Actor;
use crate::component::Inertia;
use crate::model::ActorAction;
use crate::model::ActorActionsExt;
use crate::util::ext::TransformExt;
use crate::util::math;
use bevy::ecs::system::Query;
use bevy::math::Vec2;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Res;
use bevy::prelude::Time;
use bevy::prelude::Vec3;
use bevy::transform::components::Transform;

const TURN_EPSILON: f32 = 0.01;

pub fn actor(mut query: Query<(&Actor, &mut Transform, &mut Inertia)>, time: Res<Time>) {
    let time_delta = time.delta_seconds();

    for (actor, mut transform, mut inertia) in query.iter_mut() {
        turn(actor, &mut transform, &mut inertia, time_delta);

        if actor.actions.is_empty() {
            continue;
        }

        let mut movement = Vec3::new(0.0, 0.0, 0.0);

        if actor.actions.contains(ActorAction::MovementForward) {
            movement.x += 1.0;
        }

        if actor.actions.contains(ActorAction::MovementBackward) {
            movement.x -= 1.0;
        }

        if actor.actions.contains(ActorAction::MovementLeftward) {
            movement.y += 1.0;
        }

        if actor.actions.contains(ActorAction::MovementRightward) {
            movement.y -= 1.0;
        }

        movement = transform.rotation
            * normalize_movement(movement)
            * actor.config.movement_velocity
            * time_delta;

        if actor.actions.is_sprinting() {
            movement *= actor.config.sprint_factor;
        }

        inertia.push(movement.xy(), 0.0, false, true, false);
    }
}

fn turn(actor: &Actor, transform: &mut Transform, inertia: &mut Inertia, time_delta: f32) {
    let look_at = if let Some(look_at) = actor.look_at {
        look_at
    } else {
        return;
    };

    let distance = math::angle_difference(transform.direction(), look_at);

    if distance.abs() < TURN_EPSILON {
        return;
    }

    let mut velocity = actor.config.rotation_velocity * time_delta * distance.signum();
    let velocity_current = inertia.velocity_angular;
    let velocity_future = velocity + velocity_current;
    let distance_future = velocity_future / Inertia::DRAG_ANGULAR;
    let distance_excess = distance_future / distance;

    if distance_excess > 1.0 {
        velocity /= distance_excess;
    }

    inertia.push(Vec2::ZERO, velocity, false, true, false);
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
