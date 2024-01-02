use crate::{model::AudioPlay, resource::AudioTracker, util::Timer};
use bevy::{
    ecs::{
        schedule::{IntoSystemConfigs, SystemConfigs},
        system::{Local, Res},
    },
    prelude::ResMut,
    time::Time,
};
use rand::Rng as _;
use std::{ops::Range, time::Duration};

const INTERVAL: Range<Duration> = Duration::from_secs(15)..Duration::from_secs(25);

fn ambience_fx_inner(mut audio: ResMut<AudioTracker>) {
    audio.queue(AudioPlay {
        path: "sounds/ambience_fx".into(),
        volume: 0.3,
        ..AudioPlay::DEFAULT
    });
}

pub fn ambience_fx() -> SystemConfigs {
    return ambience_fx_inner.run_if(|mut r: Local<Timer>, t: Res<Time>| {
        return r.next_if_ready(t.elapsed(), || rand::thread_rng().gen_range(INTERVAL));
    });
}
