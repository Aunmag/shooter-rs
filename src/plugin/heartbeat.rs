use crate::{
    component::Actor,
    model::{AppState, AudioPlay},
    plugin::{AudioTracker, CameraTarget, Health},
    util::{ext::AppExt, math::interpolate, Timer},
};
use bevy::{
    app::{App, Plugin},
    audio::AudioSink,
    ecs::{
        component::Component,
        system::{Local, Query},
    },
    prelude::{AudioSinkPlayback, IntoSystemConfigs, Res, With},
    time::Time,
};
use std::time::Duration;

const VOLUME: f32 = 0.7;
const SPEED_MIN: f32 = 1.4;
const SPEED_MAX: f32 = 1.8;
const UPDATE_INTERVAL: Duration = Duration::from_millis(1200);

pub struct HeartbeatPlugin;

impl Plugin for HeartbeatPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system_enter(AppState::Game, on_enter);
        app.add_state_system(
            AppState::Game,
            on_update.run_if(|mut r: Local<Timer>, t: Res<Time>| {
                r.next_if_ready(t.elapsed(), || UPDATE_INTERVAL)
            }),
        );
    }
}

#[derive(Component)]
pub struct Heartbeat;

impl Heartbeat {
    pub const PATH: &'static str = "sounds/heartbeat";
}

fn on_enter(audio: Res<AudioTracker>) {
    audio.queue(AudioPlay {
        path: Heartbeat::PATH.into(),
        duration: AudioPlay::DURATION_FOREVER,
        ..AudioPlay::DEFAULT
    });
}

fn on_update(
    heartbeats: Query<&AudioSink, With<Heartbeat>>,
    targets: Query<(&Health, &Actor), With<CameraTarget>>,
) {
    let mut play = false;
    let mut volume = 0.0;
    let mut speed = 0.0;

    for (health, actor) in targets.iter() {
        if health.is_alive() && health.is_low() {
            volume = f32::max(volume, VOLUME * (1.0 - health.get()));
            speed = f32::max(
                speed,
                interpolate(SPEED_MIN, SPEED_MAX, 1.0 - actor.stamina.powf(4.0)),
            );
            play = true;
        } else {
            play = false;
        }
    }

    for heartbeat in heartbeats.iter() {
        if play {
            heartbeat.set_volume(volume);
            heartbeat.set_speed(speed);

            if heartbeat.is_paused() {
                heartbeat.play();
            }
        } else if !heartbeat.is_paused() {
            heartbeat.pause();
        }
    }
}
