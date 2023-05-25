use crate::{
    component::{Actor, Bot},
    model::{ActorAction, ActorActions},
    util::ext::Vec2Ext,
};
use bevy::{
    math::Vec3Swizzles,
    prelude::{Query, Transform},
};

pub fn target_follow(mut bots: Query<(&Bot, &mut Actor, &Transform)>) {
    for (bot, mut actor, transform) in bots.iter_mut() {
        if let Some(target) = bot.target_final {
            actor.actions = ActorAction::MovementForward | ActorAction::Attack;
            actor.look_at = Some(transform.translation.xy().angle_to(target));
        } else {
            actor.actions = ActorActions::EMPTY;
            actor.look_at = None;
        }
    }
}
