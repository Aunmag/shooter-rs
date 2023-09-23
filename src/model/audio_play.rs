use crate::util::{ext::Vec2Ext, SmartString};
use bevy::{math::Vec2, prelude::PlaybackSettings};
use std::time::Duration;

#[derive(Clone)]
pub struct AudioPlay {
    pub path: SmartString<'static>,
    pub volume: f32,
    pub source: Option<Vec2>,
    pub duration: Duration,
}

impl AudioPlay {
    const CLOSE_DISTANCE: f32 = 0.5;

    pub const DEFAULT: Self = Self {
        path: SmartString::Ref("sound/default"),
        volume: 1.0,
        source: None,
        duration: Duration::ZERO,
    };

    pub fn settings(&self) -> PlaybackSettings {
        let settings = if self.duration.is_zero() {
            PlaybackSettings::ONCE
        } else {
            PlaybackSettings::LOOP
        };

        return settings.with_volume(self.volume);
    }

    /// Returns none if audio should be played just once or forever
    pub fn duration_limit(&self) -> Option<Duration> {
        if self.duration.is_zero() || self.duration == Duration::MAX {
            return None;
        } else {
            return Some(self.duration);
        }
    }

    pub fn is_similar_to(&self, other: &Self) -> bool {
        return self.path == other.path
            && self.is_close_to(other)
            && self.has_same_repeat_mode(other);
    }

    pub fn is_close_to(&self, other: &Self) -> bool {
        match (self.source, other.source) {
            (Some(s1), Some(s2)) => {
                return (s1 - s2).is_shorter_than(Self::CLOSE_DISTANCE);
            }
            (None, None) => {
                return false;
            }
            _ => {
                return false;
            }
        }
    }

    pub fn has_same_repeat_mode(&self, other: &Self) -> bool {
        return self.duration.is_zero() && other.duration.is_zero();
    }
}
