use crate::util::SmartString;
use bevy::{
    audio::{PlaybackMode, Volume},
    math::Vec2,
    prelude::PlaybackSettings,
};
use std::time::Duration;

#[derive(Clone)]
pub struct AudioPlay {
    pub path: SmartString<'static>,
    pub volume: f32,
    /// How quickly the sound fades with distance. Higher values make the sound fade out faster
    pub falloff: f32,
    pub speed: f32,
    pub source: Option<Vec2>,
    pub duration: Duration,
}

impl AudioPlay {
    pub const VOLUME_MIN: f32 = 0.01;

    pub const DURATION_ONCE: Duration = Duration::ZERO;
    pub const DURATION_FOREVER: Duration = Duration::MAX;

    pub const FALLOFF_SHORTEST: f32 = 0.2;
    pub const FALLOFF_SHORTER: f32 = f32::midpoint(Self::FALLOFF_MEDIUM, Self::FALLOFF_SHORTEST);
    pub const FALLOFF_MEDIUM: f32 = 0.15;
    pub const FALLOFF_LONGER: f32 = f32::midpoint(Self::FALLOFF_MEDIUM, Self::FALLOFF_LONGEST);
    pub const FALLOFF_LONGEST: f32 = 0.045;

    pub const DEFAULT: Self = Self {
        path: SmartString::Ref("sound/default"),
        volume: 1.0,
        falloff: Self::FALLOFF_MEDIUM,
        speed: 1.0,
        source: None,
        duration: Self::DURATION_ONCE,
    };

    pub fn calc_spatial_volume(&self, volume: f32, source: Vec2, listener: Vec2) -> f32 {
        return volume * (-source.distance(listener) * self.falloff).exp();
    }

    pub fn settings(&self) -> PlaybackSettings {
        return PlaybackSettings {
            mode: if self.is_looped() {
                PlaybackMode::Loop
            } else {
                PlaybackMode::Despawn
            },
            volume: Volume::Linear(self.volume),
            speed: self.speed,
            ..Default::default()
        };
    }

    pub fn is_looped(&self) -> bool {
        return !self.duration.is_zero();
    }

    pub fn is_looped_forever(&self) -> bool {
        return self.duration == Self::DURATION_FOREVER;
    }

    pub fn duration(&self) -> Option<Duration> {
        if self.is_looped() && !self.is_looped_forever() {
            return Some(self.duration);
        } else {
            return None;
        }
    }
}
