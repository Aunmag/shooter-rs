use crate::util::{ext::Vec2Ext, SmartString};
use bevy::{
    audio::{Volume, VolumeLevel},
    math::Vec2,
    prelude::PlaybackSettings,
};
use std::time::Duration;

#[derive(Clone)]
pub struct AudioPlay {
    pub path: SmartString<'static>,
    pub volume: f32,
    pub speed: f32,
    pub source: Option<Vec2>,
    pub duration: Duration,
}

impl AudioPlay {
    pub const DURATION_ONCE: Duration = Duration::ZERO;
    pub const DURATION_FOREVER: Duration = Duration::MAX;

    const CLOSE_DISTANCE: f32 = 0.5;

    pub const DEFAULT: Self = Self {
        path: SmartString::Ref("sound/default"),
        volume: 1.0,
        speed: 1.0,
        source: None,
        duration: Self::DURATION_ONCE,
    };

    pub fn settings(&self) -> PlaybackSettings {
        let settings = if self.is_looped() {
            PlaybackSettings::LOOP
        } else {
            PlaybackSettings::ONCE
        };

        return settings
            .with_volume(Volume::Relative(VolumeLevel::new(self.volume)))
            .with_speed(self.speed);
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

    pub fn is_similar_to(&self, other: &Self) -> bool {
        return self.path == other.path
            && self.has_same_source(other)
            && self.is_looped() == other.is_looped();
    }

    pub fn has_same_source(&self, other: &Self) -> bool {
        match (self.source, other.source) {
            (Some(s1), Some(s2)) => {
                return s1.is_close(s2, Self::CLOSE_DISTANCE);
            }
            (None, None) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}
