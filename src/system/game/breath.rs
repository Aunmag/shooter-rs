use crate::{
    command::AudioPlay,
    component::{Actor, Breath},
    util::math::interpolate,
};
use bevy::{
    ecs::system::Query,
    math::Vec3Swizzles,
    prelude::{Commands, Res, Time, Transform},
};
use std::time::Duration;

const BREATH_INTERVAL_MIN: Duration = Duration::from_millis(1100);
const BREATH_INTERVAL_MAX: Duration = Duration::from_millis(2200);

pub fn breath(
    mut query: Query<(&mut Breath, &Actor, &Transform)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (mut breath, actor, transform) in query.iter_mut() {
        let intensity = 1.0 - actor.stamina;

        if intensity > 0.0 && time > breath.last + calc_interval(intensity) {
            commands.add(AudioPlay {
                path: "sounds/breath_{n}.ogg",
                volume: 0.3 * intensity,
                source: Some(transform.translation.xy()),
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
