use crate::{
    plugin::{Actor, AudioPlay, AudioTracker, DebugTweaks, Health, camera_target::CameraTarget}, state::AppState, util::{Timer, ext::{AppExt, Fuzz}, math::interpolate},
};
use bevy::{
    app::{App, Plugin},
    audio::{AudioSink, Volume},
    ecs::{
        component::Component,
        system::{Local, Query},
    },
    prelude::{AudioSinkPlayback, IntoScheduleConfigs, Res, With},
    time::Time,
};
use rand::RngExt;
use rand_distr::num_traits::Signed;
use std::time::Duration;

const BPM_MIN: f32 = 70.0;
const BPM_MAX: f32 = 150.0;
const VOLUME_MIN: f32 = 0.4;
const VOLUME_MAX: f32 = 0.8;
const SPEED_MIN: f32 = 1.1;
const SPEED_MAX: f32 = 1.2;
const FOO: f32 = 0.35;

// 70: 0.6 .. 0.8
// 150: 0.8 .. 1.0
const S2_VOLUME_FACTOR: f32 = 0.8;
const S2_SPEED_FACTOR: f32 = 1.05;

const FUZZ_SPEED: f32 = 0.05;
const FUZZ_VOLUME: f32 = 0.05;
const FUZZ_INTERVAL: Duration = Duration::from_millis(10); // TODO: rename

pub struct HeartbeatPlugin;

impl Plugin for HeartbeatPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update);
    }
}

#[derive(Default, Component)]
pub struct Heartbeat {
    last_time: Duration,
    is_s1: bool,
}

fn on_update(
    mut query: Query<(&mut Heartbeat, &Health)>,
    audio_pool: Res<AudioTracker>,
    time: Res<Time>,
    tweaks: Res<DebugTweaks>,
) {
    let time = time.elapsed();

    for (mut heartbeat, health) in query.iter_mut() {
        if !health.is_alive() {
            continue;
        }

        let intensity = 1.0 - health.get() / Health::LOW_VALUE;

        if intensity <= 0.0 {
            continue;
        }

        let bpm = interpolate(BPM_MIN, BPM_MAX, intensity);

        // TODO: rename
        let foo = if heartbeat.is_s1 {
            1.0 - FOO
        } else {
            FOO
        };

        let interval = Duration::from_secs_f32(60.0 / bpm * foo);

        if heartbeat.last_time + interval > time {
            continue;
        }

        let mut audio = AudioPlay {
            volume: interpolate(VOLUME_MIN, VOLUME_MAX, intensity),
            speed: interpolate(SPEED_MIN, SPEED_MAX, intensity),
            ..AudioPlay::DEFAULT
        };

        if heartbeat.is_s1 {
            audio.path = "sounds/player/heartbeat_s1".into();
        } else {
            audio.path = "sounds/player/heartbeat_s2".into();
            audio.volume *= S2_VOLUME_FACTOR;
            audio.speed *= S2_SPEED_FACTOR;
        };

        let mut rng = rand::rng();
        heartbeat.last_time = time;

        audio.speed += rng.random_range(-FUZZ_SPEED..FUZZ_SPEED);
        audio.volume = audio.volume.fuzz_with(&mut rng, FUZZ_VOLUME);

        let foo = rng.random_range(Duration::ZERO..FUZZ_INTERVAL); // TODO: rename
        if rng.random() {
            heartbeat.last_time += foo;
        } else {
            heartbeat.last_time = heartbeat.last_time.saturating_sub(foo);
        }

        heartbeat.is_s1 = !heartbeat.is_s1;

        audio_pool.queue(audio);
    }
}
