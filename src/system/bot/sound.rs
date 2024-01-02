use crate::{component::Bot, model::AudioPlay, resource::AudioTracker};
use bevy::{
    ecs::system::Res,
    math::Vec3Swizzles,
    prelude::{Query, ResMut, Transform},
    time::Time,
};
use rand::Rng as _;
use std::{ops::Range, time::Duration};

const INTERVAL: Range<Duration> = Duration::from_secs(2)..Duration::from_secs(30);

pub fn sound(
    mut bots: Query<(&mut Bot, &Transform)>,
    mut audio: ResMut<AudioTracker>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (mut bot, transform) in bots.iter_mut() {
        if !bot
            .voice_timer
            .next_if_ready(time, || rand::thread_rng().gen_range(INTERVAL))
        {
            continue;
        }

        audio.queue(AudioPlay {
            path: "actors/zombie/misc".into(),
            volume: 0.7,
            source: Some(transform.translation.xy()),
            ..AudioPlay::DEFAULT
        });
    }
}
