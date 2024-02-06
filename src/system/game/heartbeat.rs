use crate::{
    component::{Actor, Health, Heartbeat, Player},
    util::{math::interpolate, Timer},
};
use bevy::{
    audio::AudioSink,
    ecs::{
        schedule::SystemConfigs,
        system::{Local, Query},
    },
    prelude::{AudioSinkPlayback, IntoSystemConfigs, Res, With},
    time::Time,
};
use std::time::Duration;

const VOLUME: f32 = 0.7;
const SPEED_MIN: f32 = 1.3;
const SPEED_MAX: f32 = 1.9;
const UPDATE_INTERVAL: Duration = Duration::from_millis(1500);

pub fn heartbeat_inner(
    // TODO: simplify
    mut heartbeats: Query<&AudioSink, With<Heartbeat>>,
    players: Query<(&Health, &Actor), With<Player>>,
) {
    for heartbeat in heartbeats.iter_mut() {
        if let Some((health, actor)) = players.iter().next() {
            if health.is_alive() && health.is_low() {
                let speed = interpolate(SPEED_MIN, SPEED_MAX, 1.0 - actor.stamina.powf(4.0));
                heartbeat.set_volume(VOLUME * (1.0 - health.get_normalized()));
                heartbeat.set_speed(speed);

                if heartbeat.is_paused() {
                    heartbeat.play();
                }
            } else {
                heartbeat.pause();
            }
        }
    }
}

pub fn heartbeat() -> SystemConfigs {
    return heartbeat_inner.run_if(|mut r: Local<Timer>, t: Res<Time>| {
        return r.next_if_ready(t.elapsed(), || UPDATE_INTERVAL);
    });
}
