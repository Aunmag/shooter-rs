use crate::util::ext::Vec2Ext;
use bevy::{math::Vec2, prelude::PlaybackSettings};
use std::time::Duration;

#[derive(Clone)]
pub struct AudioPlay {
    pub path: &'static str,
    pub volume: f32,
    pub chance: f32,
    pub source: Option<Vec2>,
    pub duration: Duration,
    pub priority: u8,
}

impl AudioPlay {
    pub const PRIORITY_LOWEST: u8 = 0;
    pub const PRIORITY_LOWER: u8 = 1;
    pub const PRIORITY_MEDIUM: u8 = 2;
    pub const PRIORITY_HIGHER: u8 = 3;
    pub const PRIORITY_HIGHEST: u8 = 4;

    const CLOSE_DISTANCE: f32 = 0.5;

    pub const DEFAULT: Self = Self {
        path: "sounds/default.ogg",
        volume: 1.0,
        chance: 1.0,
        source: None,
        duration: Duration::ZERO,
        priority: Self::PRIORITY_MEDIUM,
    };

    pub fn as_spatial(&self, source: Vec2) -> Self {
        return Self {
            source: Some(source),
            ..self.clone()
        };
    }

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
        return std::ptr::eq(self.path, other.path)
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
