use crate::component::Actor;
use crate::component::ActorActions;
use crate::component::Ai;
use crate::component::Health;
use crate::util::ext::Vec2Ext;
use crate::util::math;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use bevy::prelude::With;
use rand::Rng;
use rand_pcg::Pcg32;
use std::f32::consts::FRAC_PI_8;

const CHANGE_MOVEMENT_PROBABILITY: f64 = 1.0;
const TURN_PROBABILITY: f64 = 2.0;
const MAX_DISTANCE_FROM_CENTER: f32 = 7.5;

pub fn ai(
    mut query: Query<(&Health, &Transform, &mut Actor), With<Ai>>,
    time: Res<Time>,
    mut randomizer: ResMut<Pcg32>,
) {
    let delta = time.delta_seconds() as f64;
    let change_movement_probability = CHANGE_MOVEMENT_PROBABILITY * delta;
    let turn_probability = TURN_PROBABILITY * delta;

    for (health, transform, mut actor) in query.iter_mut() {
        if !health.is_alive() {
            actor.actions = ActorActions::empty();
            continue;
        }

        if gen_chance(&mut randomizer, change_movement_probability) {
            actor.actions.toggle(ActorActions::MOVEMENT_FORWARD);
        }

        let is_moving = actor.actions.contains(ActorActions::MOVEMENT_FORWARD);

        if !is_moving && gen_chance(&mut randomizer, turn_probability) {
            let position = transform.translation.xy();

            if position.is_shorter_than(MAX_DISTANCE_FROM_CENTER) {
                actor.look_at += randomizer.gen_range(-FRAC_PI_8..FRAC_PI_8);
            } else {
                actor.look_at += math::angle_difference(position.direction(), transform.rotation.z);
            }
        }
    }
}

// TODO: to ext method
fn gen_chance(randomizer: &mut Pcg32, probability: f64) -> bool {
    if probability >= 1.0 {
        return true;
    } else if probability <= 0.0 {
        return false;
    } else {
        return randomizer.gen_bool(probability);
    }
}
