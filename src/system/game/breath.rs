use crate::{
    component::{Actor, Breath},
    model::AudioPlay,
    resource::AudioTracker,
    util::math::interpolate,
};
use bevy::{
    ecs::system::{Query, ResMut},
    math::Vec3Swizzles,
    prelude::{Res, Time, Transform},
};
use std::time::Duration;

const BREATH_INTERVAL_MIN: Duration = Duration::from_millis(1100);
const BREATH_INTERVAL_MAX: Duration = Duration::from_millis(2200);

pub fn breath(
    mut query: Query<(&mut Breath, &Actor, &Transform)>,
    mut audio: ResMut<AudioTracker>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (mut breath, actor, transform) in query.iter_mut() {
        let intensity = 1.0 - actor.stamina;

        if intensity > 0.0 && time > breath.last + calc_interval(intensity) {
            audio.queue(AudioPlay {
                path: "sounds/breath_{n}.ogg",
                volume: 0.24 * intensity,
                source: Some(transform.translation.xy()),
                priority: AudioPlay::PRIORITY_LOWEST,
                ..AudioPlay::DEFAULT
            });

            breath.last = time;
        }
    }
}

fn calc_interval(intensity: f32) -> Duration {
    let interval = interpolate(
        BREATH_INTERVAL_MIN.as_secs_f32(),
        BREATH_INTERVAL_MAX.as_secs_f32(),
        1.0 - intensity, // the higher intensity, the shorter interval
    );
    return Duration::from_secs_f32(interval);
}
