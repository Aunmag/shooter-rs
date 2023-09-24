use crate::{
    component::{Actor, Health, Heartbeat, Player},
    util::math::interpolate,
};
use bevy::{
    audio::AudioSink,
    ecs::system::Query,
    prelude::{AudioSinkPlayback, Res, With},
    time::Time,
};
use std::time::Duration;

const VOLUME: f32 = 0.7;
const SPEED_MIN: f32 = 1.3;
const SPEED_MAX: f32 = 1.9;
const RUN_INTERVAL: Duration = Duration::from_millis(1500);

pub fn heartbeat(
    mut heartbeats: Query<(&mut Heartbeat, &AudioSink)>,
    players: Query<(&Health, &Actor), With<Player>>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (mut heartbeat, audio) in heartbeats.iter_mut() {
        if time < heartbeat.next {
            continue;
        }

        if let Some((health, actor)) = players.iter().next() {
            if health.is_low() {
                let speed = interpolate(SPEED_MIN, SPEED_MAX, 1.0 - actor.stamina.powf(4.0));
                audio.set_volume(VOLUME * (1.0 - health.get()));
                audio.set_speed(speed);

                if audio.is_paused() {
                    audio.play();
                }
            } else {
                audio.pause();
            }
        }

        heartbeat.next = time + RUN_INTERVAL;
    }
}
