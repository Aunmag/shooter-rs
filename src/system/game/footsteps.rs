use crate::command::AudioPlay;
use crate::component::Footsteps;
use crate::util::math::interpolate;
use bevy::ecs::system::Query;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Commands;
use bevy::prelude::Res;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use std::time::Duration;

const STRIDE_DISTANCE_MIN: f32 = 0.1;
const STRIDE_RATE_MIN: (f32, f32, f32) = (0.1, 70.0, 0.02);
const STRIDE_RATE_MAX: (f32, f32, f32) = (5.0, 135.0, 0.2);

// TODO: play sound on turn
pub fn footsteps(
    mut query: Query<(&mut Footsteps, &Transform)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (mut footsteps, transform) in query.iter_mut() {
        let translation = transform.translation.xy();
        let distance_squared = footsteps.position.distance_squared(translation);

        if distance_squared < STRIDE_DISTANCE_MIN * STRIDE_DISTANCE_MIN {
            continue;
        }

        let distance = distance_squared.sqrt();
        let velocity = distance / (time - footsteps.time).as_secs_f32();
        let intensity = calc_stride_intensity(velocity);

        if time < footsteps.time + calc_stride_interval(intensity) {
            continue;
        }

        commands.add(AudioPlay {
            path: "sounds/footstep_{n}.ogg",
            volume: calc_stride_volume(intensity),
            source: Some(translation),
            choices: 8,
            ..Default::default()
        });

        footsteps.time = time;
        footsteps.position = translation;
    }
}

fn calc_stride_intensity(velocity: f32) -> f32 {
    return (velocity / (STRIDE_RATE_MAX.0 - STRIDE_RATE_MIN.0)).clamp(0.0, 1.0);
}

fn calc_stride_interval(intensity: f32) -> Duration {
    let rate = interpolate(STRIDE_RATE_MIN.1, STRIDE_RATE_MAX.1, intensity);
    return Duration::from_secs_f32(60.0 / rate);
}

fn calc_stride_volume(intensity: f32) -> f32 {
    return interpolate(STRIDE_RATE_MIN.2, STRIDE_RATE_MAX.2, intensity);
}
