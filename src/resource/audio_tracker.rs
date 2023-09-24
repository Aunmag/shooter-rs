use crate::model::AudioPlay;
use bevy::prelude::{Resource, Vec2};

const VOLUME_MIN: f32 = 0.01;
const SOUND_DISTANCE_FACTOR: f32 = 2.0;

#[derive(Resource)]
pub struct AudioTracker {
    queue: Vec<AudioPlay>,
    limit: usize,
    pub playing: usize,
    pub listener: Vec2,
}

impl AudioTracker {
    pub fn new(sources_limit: usize) -> Self {
        return Self {
            queue: Vec::new(),
            playing: 0,
            limit: sources_limit,
            listener: Vec2::ZERO,
        };
    }

    pub fn queue(&mut self, mut audio: AudioPlay) {
        if let Some(source) = audio.source {
            audio.volume = self.calc_spatial_volume(source, audio.volume);
        }

        if audio.volume < VOLUME_MIN {
            return;
        }

        let is_overflow = self.playing + self.queue.len() >= self.limit;
        let mut replacement = None;

        for (i, other) in self.queue.iter().enumerate() {
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
                self.queue[i] = audio;
            }
        } else {
            self.queue.push(audio);
        }
    }

    pub fn pop_queued(&mut self) -> Option<AudioPlay> {
        return self.queue.pop();
    }

    pub fn calc_spatial_volume(&self, source: Vec2, volume: f32) -> f32 {
        return f32::min(SOUND_DISTANCE_FACTOR / source.distance(self.listener), 1.0) * volume;
    }
}
