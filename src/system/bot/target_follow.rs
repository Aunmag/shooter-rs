use crate::component::Actor;
use crate::component::ActorActions;
use crate::component::Bot;
use crate::util::ext::Vec2Ext;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Query;
use bevy::prelude::Transform;
use std::f32::consts::FRAC_PI_2;

pub fn target_follow(mut bots: Query<(&Bot, &mut Actor, &Transform)>) {
    for (bot, mut actor, transform) in bots.iter_mut() {
        if let Some(target) = bot.target_final {
            actor.actions = ActorActions::MOVEMENT_FORWARD;
            actor.look_at = transform.translation.xy().atan2_to(target) + FRAC_PI_2;
        } else {
            actor.actions = ActorActions::empty();
        }
    }
}
