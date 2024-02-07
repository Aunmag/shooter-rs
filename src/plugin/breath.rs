use crate::{
    component::Actor,
    model::{AppState, AudioPlay},
    resource::AudioTracker,
    util::{ext::AppExt, math::interpolate},
};
use bevy::{
    app::{App, Plugin},
    ecs::{component::Component, system::Query},
    math::Vec3Swizzles,
    prelude::{Res, Time, Transform},
};
use std::time::Duration;

const BREATH_INTERVAL_MIN: Duration = Duration::from_millis(1100);
const BREATH_INTERVAL_MAX: Duration = Duration::from_millis(2200);

pub struct BreathPlugin;

impl Plugin for BreathPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update);
    }
}

#[derive(Default, Component)]
pub struct Breath {
    last: Duration,
}

fn on_update(
    mut query: Query<(&mut Breath, &Actor, &Transform)>,
    audio: Res<AudioTracker>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (mut breath, actor, transform) in query.iter_mut() {
        let intensity = 1.0 - actor.stamina;

        if intensity > 0.0 && time > breath.last + calc_interval(intensity) {
            audio.queue(AudioPlay {
                path: "sounds/breath".into(),
                volume: 0.26 * intensity,
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
