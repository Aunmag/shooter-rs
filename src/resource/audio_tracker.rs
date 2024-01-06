use crate::model::AudioPlay;
use bevy::prelude::{Resource, Vec2};
use std::sync::Mutex;

const VOLUME_MIN: f32 = 0.01;
const SOUND_DISTANCE_FACTOR: f32 = 2.0;

#[derive(Resource)]
pub struct AudioTracker {
    queue: Mutex<Vec<AudioPlay>>,
    limit: usize,
    pub playing: usize,
    pub listener: Vec2,
}

impl AudioTracker {
    pub fn new(sources_limit: usize) -> Self {
        return Self {
            queue: Mutex::new(Vec::with_capacity(sources_limit)),
            playing: 0,
            limit: sources_limit,
            listener: Vec2::ZERO,
        };
    }

    pub fn queue(&self, mut audio: AudioPlay) {
        if let Some(source) = audio.source {
            audio.volume = self.calc_spatial_volume(source, audio.volume);
        }

        if audio.volume < VOLUME_MIN {
            return;
        }

        let Ok(mut queue) = self.queue.lock() else {
            log::error!("Unable to queue audio. Audio tracker is poisoned");
            return;
        };

        let is_overflow = self.playing + queue.len() >= self.limit;
        let mut replacement = None;

        for (i, other) in queue.iter().enumerate() {
            if audio.is_similar_to(other) {
                return;
            }

            if is_overflow
                && other.volume < audio.volume
                && replacement.map_or(true, |(_, v)| other.volume > v)
            {
                replacement = Some((i, other.volume));
            }
        }

        if is_overflow {
            if let Some((i, _)) = replacement {
                queue[i] = audio;
            }
        } else {
            queue.push(audio);
        }
    }

    pub fn take_queue(&self) -> Vec<AudioPlay> {
        if let Ok(mut queue) = self.queue.lock() {
            if queue.is_empty() {
                return Vec::new();
            } else {
                return std::mem::replace(&mut queue, Vec::with_capacity(self.limit));
            }
        } else {
            return Vec::new();
        }
    }

    pub fn calc_spatial_volume(&self, source: Vec2, volume: f32) -> f32 {
        return f32::min(SOUND_DISTANCE_FACTOR / source.distance(self.listener), 1.0) * volume;
    }
}
