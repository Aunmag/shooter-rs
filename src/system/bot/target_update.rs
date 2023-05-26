use crate::{
    component::{Actor, Bot, Inertia},
    util::{self, Interpolation, Timer},
};
use bevy::{
    ecs::system::{Res, Resource},
    math::Vec3Swizzles,
    prelude::{Query, ResMut, Transform, With},
    time::Time,
};
use std::time::Duration;

const RUN_INTERVAL: Duration = Duration::from_millis(700);

#[derive(Resource)]
pub struct TargetUpdateData {
    timer: Timer,
}

impl Default for TargetUpdateData {
    fn default() -> Self {
        return Self {
            timer: Timer::new(RUN_INTERVAL),
        };
    }
}

pub fn target_update(
    mut bots: Query<(&mut Bot, &Transform, &Inertia)>,
    actors: Query<(&Transform, &Inertia), With<Actor>>,
    mut data: ResMut<TargetUpdateData>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    if !data.timer.next_if_done(time) {
        return;
    }

    for (mut bot, bot_transform, bot_inertia) in bots.iter_mut() {
        if let Some((target_position, target_velocity)) = bot
            .target_actor
            .and_then(|e| actors.get(e).ok())
            .map(|a| (a.0.translation.xy(), a.1.velocity))
        {
            let target = util::math::find_meet_point(
                bot_transform.translation.xy(),
                bot_inertia.velocity,
                target_position,
                target_velocity,
            );

            if let Some(bot_target_final) = bot.target_point.as_mut() {
                bot_target_final.add(target, time);
            } else {
                bot.target_point = Some(Interpolation::new(target, time));
            }
        } else {
            bot.target_point = None;
        }
    }
}
