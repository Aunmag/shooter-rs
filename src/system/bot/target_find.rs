use crate::component::Actor;
use crate::component::Bot;
use crate::component::Inertia;
use crate::util;
use crate::util::ext::IteratorExt;
use crate::util::Timer;
use bevy::ecs::system::Resource;
use bevy::math::Vec2;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Entity;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Transform;
use bevy::time::Time;
use std::time::Duration;

const RUN_INTERVAL: Duration = Duration::from_secs(2);

#[derive(Resource)]
pub struct TargetFindData {
    timer: Timer,
}

impl Default for TargetFindData {
    fn default() -> Self {
        return Self {
            timer: Timer::new(RUN_INTERVAL),
        };
    }
}

pub fn target_find(
    mut bots: Query<(&mut Bot, &Actor, &Transform, &Inertia)>,
    actors: Query<(Entity, &Actor, &Transform, &Inertia)>,
    mut data: ResMut<TargetFindData>,
    time: Res<Time>,
) {
    if !data.timer.next_if_done(time.elapsed()) {
        return;
    }

    for (mut bot, bot_actor, bot_transform, bot_inertia) in bots.iter_mut() {
        let bot_position = bot_transform.translation.xy();

        bot.target_actor = actors
            .iter()
            .filter(|(_, target_actor, _, _)| {
                target_actor.config.actor_type != bot_actor.config.actor_type
            })
            .closest(|(_, _, target_transform, target_inertia)| {
                let meet_point = util::math::find_meet_point(
                    bot_position,
                    bot_inertia.velocity,
                    target_transform.translation.xy(),
                    target_inertia.velocity,
                );

                return Vec2::distance_squared(bot_position, meet_point);
            })
            .map(|v| v.0);
    }
}
