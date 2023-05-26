use crate::{
    component::{Actor, Bot},
    model::{ActorAction, ActorActions},
    util::ext::Vec2Ext,
};
use bevy::{
    ecs::system::Res,
    math::Vec3Swizzles,
    prelude::{Query, Transform},
    time::Time,
};

pub fn target_follow(mut bots: Query<(&Bot, &mut Actor, &Transform)>, time: Res<Time>) {
    let time = time.elapsed();

    for (bot, mut actor, transform) in bots.iter_mut() {
        if let Some(target) = &bot.target_point {
            let target = target.interpolate(time);
            actor.actions = ActorAction::MovementForward | ActorAction::Attack;
            actor.look_at =
                Some(transform.translation.xy().angle_to(target) + bot.direction_distortion);
        } else {
            actor.actions = ActorActions::EMPTY;
            actor.look_at = None;
        }
    }
}
