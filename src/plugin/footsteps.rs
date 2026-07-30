use crate::{
    plugin::{AudioPlay, AudioTracker},
    state::AppState,
    util::{
        ext::{AppExt, Fuzz},
        math::interpolate,
        SmartString, Timer,
    },
};
use bevy::{
    app::{App, Plugin},
    ecs::{
        component::Component,
        schedule::IntoScheduleConfigs,
        system::{Local, Query},
    },
    math::Vec2,
    prelude::{Res, Time, Transform},
};
use rand::RngExt;
use std::time::Duration;

const STRIDE_DISTANCE_MIN: f32 = 0.1;
const STRIDE_RATE_MIN: (f32, f32, f32) = (0.1, 70.0, 0.04);
const STRIDE_RATE_MAX: (f32, f32, f32) = (5.0, 135.0, 0.19);

const SOUND: AudioPlay = AudioPlay {
    path: SmartString::Ref("sounds/footstep"),
    falloff: AudioPlay::FALLOFF_SHORTEST,
    ..AudioPlay::DEFAULT
};

const BUFFER_DURATION: Duration = Duration::from_millis(10);

pub struct FootstepsPlugin;

impl Plugin for FootstepsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(
            AppState::Game,
            on_update.run_if(|mut r: Local<Timer>, t: Res<Time>| {
                return r.try_next_set(t.elapsed(), || BUFFER_DURATION);
            }),
        );
    }
}

#[derive(Default, Component)]
pub struct Footsteps {
    position: Vec2,
    time: Duration,
}

// TODO: play sound on turn
fn on_update(
    mut query: Query<(&mut Footsteps, &Transform)>,
    audio: Res<AudioTracker>,
    time: Res<Time>,
) {
    crate::util::bench::bench!();
    let time = time.elapsed();
    let mut combined_volume = 0.0;

    for (mut footsteps, transform) in query.iter_mut() {
        let position = transform.translation.truncate();

        if footsteps.time.is_zero() {
            let offset_ms = rand::rng().random_range(0..500);
            let offset = Duration::from_millis(offset_ms);
            footsteps.time = time + offset;
            footsteps.position = position;
            continue;
        }

        let elapsed = time.saturating_sub(footsteps.time);

        if elapsed.is_zero() {
            continue;
        }

        let distance_squared = footsteps.position.distance_squared(position);

        if distance_squared.is_nan() || distance_squared < STRIDE_DISTANCE_MIN * STRIDE_DISTANCE_MIN
        {
            continue;
        }

        let distance = distance_squared.sqrt();
        let velocity = distance / elapsed.as_secs_f32();
        let intensity = calc_stride_intensity(velocity);

        if time < footsteps.time + calc_stride_interval(intensity) {
            continue;
        }

        footsteps.time = time;
        footsteps.position = position;

        let volume_abstract = calc_stride_volume(intensity);
        let volume_spatial = SOUND.calc_spatial_volume(volume_abstract, position, audio.listener);

        combined_volume += volume_spatial * volume_spatial;
    }

    if combined_volume > AudioPlay::VOLUME_MIN * AudioPlay::VOLUME_MIN {
        audio.queue(AudioPlay {
            volume: f32::min(combined_volume.sqrt(), 1.0),
            speed: 1.0.fuzz_with(&mut rand::rng(), 0.1),
            ..SOUND
        });
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
