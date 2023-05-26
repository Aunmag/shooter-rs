use crate::{
    component::{Actor, Health, Player},
    resource::HeartbeatResource,
    util::math::interpolate,
};
use bevy::{
    asset::Assets,
    audio::AudioSink,
    ecs::system::Query,
    prelude::{AudioSinkPlayback, Res, ResMut, With},
    time::Time,
};
use std::time::Duration;

const VOLUME: f32 = 0.7;
const SPEED_MIN: f32 = 1.3;
const SPEED_MAX: f32 = 1.9;
const RUN_INTERVAL: Duration = Duration::from_millis(1500);

pub fn heartbeat(
    query: Query<(&Health, &Actor), With<Player>>,
    mut heartbeat: ResMut<HeartbeatResource>,
    sinks: Res<Assets<AudioSink>>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    if time < heartbeat.next {
        return;
    }

    if let Some((health, actor)) = query.iter().next() {
        if let Some(audio) = heartbeat.sink.as_ref().and_then(|h| sinks.get(h)) {
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
    }

    heartbeat.next = time + RUN_INTERVAL;
}
