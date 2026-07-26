use crate::{
    plugin::{AudioPlay, AudioTracker},
    state::AppState,
    util::{ext::AppExt, Timer},
};
use bevy::{
    ecs::{
        schedule::IntoScheduleConfigs,
        system::{Local, Res},
    },
    prelude::{App, Plugin},
    time::Time,
};
use rand::Rng as _;
use std::{ops::Range, time::Duration};

const INTERVAL: Range<Duration> = Duration::from_secs(15)..Duration::from_secs(25);

pub struct AmbienceFxPlugin;

impl Plugin for AmbienceFxPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(
            AppState::Game,
            on_update.run_if(|mut r: Local<Timer>, t: Res<Time>| {
                return r.try_next_set(t.elapsed(), || rand::thread_rng().gen_range(INTERVAL));
            }),
        );
    }
}

fn on_update(audio: Res<AudioTracker>) {
    audio.queue(AudioPlay {
        path: "sounds/ambience_fx".into(),
        volume: 0.3,
        ..AudioPlay::DEFAULT
    });
}
