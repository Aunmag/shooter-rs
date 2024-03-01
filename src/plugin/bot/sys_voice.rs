use crate::{
    component::{Actor, ActorKind},
    model::AudioPlay,
    plugin::bot::Bot,
    resource::AudioTracker,
};
use bevy::{
    ecs::system::Res,
    math::Vec3Swizzles,
    prelude::{Query, Transform},
    time::Time,
};
use rand::Rng as _;
use std::{ops::Range, time::Duration};

const INTERVAL: Range<Duration> = Duration::from_secs(5)..Duration::from_secs(30);

pub fn on_update(
    mut bots: Query<(&mut Bot, &Actor, &Transform)>,
    audio: Res<AudioTracker>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (mut bot, actor, transform) in bots.iter_mut() {
        if actor.config.kind == ActorKind::Human {
            continue;
        }

        if !bot
            .voice_timer
            .next_if_ready(time, || rand::thread_rng().gen_range(INTERVAL))
        {
            continue;
        }

        audio.queue(AudioPlay {
            path: format!("{}/misc", actor.config.get_assets_path()).into(),
            volume: 0.7,
            source: Some(transform.translation.xy()),
            ..AudioPlay::DEFAULT
        });
    }
}
