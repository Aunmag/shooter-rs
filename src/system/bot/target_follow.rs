use crate::component::Actor;
use crate::component::Bot;
use crate::model::ActorAction;
use crate::model::ActorActions;
use crate::util::ext::Vec2Ext;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Query;
use bevy::prelude::Transform;

pub fn target_follow(mut bots: Query<(&Bot, &mut Actor, &Transform)>) {
    for (bot, mut actor, transform) in bots.iter_mut() {
        if let Some(target) = bot.target_final {
            actor.actions = ActorActions::only(ActorAction::MovementForward);
            actor.look_at = Some(transform.translation.xy().angle_to(target));
        } else {
            actor.actions = ActorActions::EMPTY;
            actor.look_at = None;
        }
    }
}
