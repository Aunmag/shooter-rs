use crate::{
    component::Player,
    resource::{AudioStorage, AudioTracker},
};
use bevy::{
    asset::Assets,
    audio::AudioSink,
    ecs::{query::With, system::Command},
    math::{Vec2, Vec3Swizzles},
    prelude::{Audio, PlaybackSettings, World},
    time::Time,
    transform::components::Transform,
};
use rand::{thread_rng, Rng as _};
use std::time::Duration;

const VOLUME_MIN: f32 = 0.01;

#[derive(Clone)]
pub struct AudioPlay {
    pub path: &'static str,
    pub volume: f32,
    pub repeat: AudioRepeat,
    pub chance: f32,
    pub source: Option<Vec2>,
}

impl AudioPlay {
    pub const DEFAULT: Self = Self {
        path: "sounds/default.ogg",
        volume: 1.0,
        repeat: AudioRepeat::Once,
        chance: 1.0,
        source: None,
    };

    pub fn as_spatial(&self, source: Vec2) -> Self {
        return Self {
            source: Some(source),
            ..self.clone()
        };
    }

    pub fn should_play(&self) -> bool {
        return self.volume > VOLUME_MIN && self.chance > 0.0;
    }

    pub fn settings(&self) -> PlaybackSettings {
        let settings = if let AudioRepeat::Once = self.repeat {
            PlaybackSettings::ONCE
        } else {
            PlaybackSettings::LOOP
        };

        return settings.with_volume(self.volume);
    }
}

impl Command for AudioPlay {
    fn write(mut self, world: &mut World) {
        if !self.should_play() {
            return;
        }

        if self.chance < 1.0 && !thread_rng().gen_bool(self.chance.into()) {
            return;
        }

        if let Some(source) = self.source {
            if let Some(listener) = world
                .query_filtered::<&Transform, With<Player>>()
                .iter(world)
                .next()
            {
                self.volume = calc_spatial_volume(listener.translation.xy(), source, self.volume);
            }
        }

        let handle = if let Some(handle) = world.resource_mut::<AudioStorage>().choose(self.path) {
            handle
        } else {
            log::warn!("Audio {} not found", self.path);
            return;
        };

        let sink = world
            .resource::<Audio>()
            .play_with_settings(handle, self.settings());

        if let AudioRepeat::Loop(duration) = self.repeat {
            if !duration.is_zero() {
                let time = world.resource::<Time>().elapsed();

                let handle = world.resource::<Assets<AudioSink>>().get_handle(sink);

                world
                    .resource_mut::<AudioTracker>()
                    .track_temporary(handle, time + duration);
            }
        }
    }
}

fn calc_spatial_volume(listener: Vec2, source: Vec2, volume: f32) -> f32 {
    return f32::min(1.0 / source.distance(listener).sqrt(), 1.0) * volume;
}

#[derive(Clone)]
pub enum AudioRepeat {
    Once,
    Loop(Duration),
}
