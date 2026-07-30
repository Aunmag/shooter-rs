use crate::{
    plugin::{Actor, AudioPlay, AudioTracker},
    state::AppState,
    util::{ext::AppExt, Timer},
};
use bevy::{
    ecs::{component::Component, system::Query},
    math::Vec3Swizzles,
    prelude::{App, Plugin, Res, Time},
    transform::components::Transform,
};
use rand::RngExt;
use std::{ops::Range, time::Duration};

const INTERVAL: Range<Duration> = Duration::from_secs(5)..Duration::from_secs(30);

pub struct BotVoicePlugin;

impl Plugin for BotVoicePlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update);
    }
}

#[derive(Default, Component)]
pub struct BotVoice {
    timer: Timer,
}

fn on_update(
    mut bots: Query<(&mut BotVoice, &Actor, &Transform)>,
    audio: Res<AudioTracker>,
    time: Res<Time>,
) {
    crate::util::bench::bench!();
    let time = time.elapsed();

    for (mut voice, actor, transform) in bots.iter_mut() {
        if !voice
            .timer
            .try_next_set(time, || rand::rng().random_range(INTERVAL))
        {
            continue;
        }

        audio.queue(AudioPlay {
            path: format!("{}/misc", actor.config.get_assets_path()).into(),
            volume: 0.7,
            source: Some(transform.translation.xy()),
            ..AudioPlay::DEFAULT
        });
    }
}
