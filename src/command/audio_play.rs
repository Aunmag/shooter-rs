use crate::{
    component::Player,
    resource::{AudioStorage, AudioTracker},
};
use bevy::{
    asset::Assets,
    audio::AudioSink,
    ecs::{query::With, system::Command},
    math::{Vec2, Vec3Swizzles},
    prelude::{Audio, AudioSinkPlayback, PlaybackSettings, World},
    transform::components::Transform,
};
use rand::{thread_rng, Rng as _};
use std::time::Duration;

const VOLUME_MIN: f32 = 0.01;
const DEFAULT_DURATION: Duration = Duration::from_secs(1);

#[derive(Clone)]
pub struct AudioPlay {
    pub path: &'static str,
    pub volume: f32,
    pub repeat: AudioRepeat,
    pub chance: f32,
    pub source: Option<Vec2>,
    pub priority: u8,
}

impl AudioPlay {
    pub const PRIORITY_LOWEST: u8 = 0;
    pub const PRIORITY_LOWER: u8 = 1;
    pub const PRIORITY_MEDIUM: u8 = 2;
    pub const PRIORITY_HIGHER: u8 = 3;
    pub const PRIORITY_HIGHEST: u8 = 4;

    pub const DEFAULT: Self = Self {
        path: "sounds/default.ogg",
        volume: 1.0,
        repeat: AudioRepeat::Once,
        chance: 1.0,
        source: None,
        priority: Self::PRIORITY_MEDIUM,
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

        let (has_space, removed) = world
            .resource_mut::<AudioTracker>()
            .provide_space(self.priority, self.volume);

        if !has_space {
            debug_assert!(removed.is_none());
            return;
        }

        if let Some(removed) = removed {
            if let Some(sink) = world.resource::<Assets<AudioSink>>().get(&removed) {
                sink.stop();
            }
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

        let audio_source =
            if let Some(handle) = world.resource_mut::<AudioStorage>().choose(self.path) {
                handle
            } else {
                log::warn!("Audio {} not found", self.path);
                return;
            };

        let audio_sink = world
            .resource::<Audio>()
            .play_with_settings(audio_source, self.settings());

        let audio_sink_played = world.resource::<Assets<AudioSink>>().get_handle(audio_sink);

        let duration = self.repeat.duration().unwrap_or(DEFAULT_DURATION); // TODO: find real duration

        world.resource_mut::<AudioTracker>().register(
            audio_sink_played,
            self.priority,
            self.volume,
            duration,
            self.repeat.has_duration(),
        );
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

impl AudioRepeat {
    pub fn has_duration(&self) -> bool {
        if let Self::Loop(duration) = self {
            return !duration.is_zero();
        } else {
            return false;
        }
    }

    pub fn duration(&self) -> Option<Duration> {
        if let Self::Loop(duration) = self {
            if duration.is_zero() {
                return None;
            } else {
                return Some(*duration);
            }
        } else {
            return None;
        }
    }
}
