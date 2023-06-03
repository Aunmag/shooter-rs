use crate::{model::AudioPlay, resource::AudioTracker};
use bevy::{
    ecs::system::{Res, Resource},
    prelude::ResMut,
    time::Time,
};
use rand::{thread_rng, Rng as _};
use std::time::Duration;

const INTERVAL_MIN: f32 = 15.0;
const INTERVAL_MAX: f32 = 25.0;

#[derive(Default, Resource)]
pub struct AmbienceFxData {
    next: Duration,
}

pub fn ambience_fx(
    mut data: ResMut<AmbienceFxData>,
    mut audio: ResMut<AudioTracker>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    if time < data.next {
        return;
    }

    if !data.next.is_zero() {
        audio.queue(AudioPlay {
            path: "sounds/ambience_fx_{n}.ogg",
            volume: 0.3,
            priority: AudioPlay::PRIORITY_MEDIUM,
            ..AudioPlay::DEFAULT
        });
    }

    let interval = Duration::from_secs_f32(thread_rng().gen_range(INTERVAL_MIN..INTERVAL_MAX));
    data.next = time + interval;
}
