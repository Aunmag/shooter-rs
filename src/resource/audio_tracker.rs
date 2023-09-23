use super::AudioStorage;
use crate::model::AudioPlay;
use bevy::prelude::{Assets, Audio, AudioSink, AudioSinkPlayback, Handle, Resource, Vec2};
use std::time::{Duration, Instant};

const VOLUME_MIN: f32 = 0.01;
const AUDIO_DURATION: Duration = Duration::from_secs(1); // TODO: find real duration
/// Just in case to prevent clipping
const AUDIO_DURATION_EXTRA: Duration = Duration::from_millis(50);
const SOUND_DISTANCE_FACTOR: f32 = 2.0;

#[derive(Resource)]
pub struct AudioTracker {
    sources_limit: usize,
    queue: Vec<AudioPlay>,
    playing: Vec<Source>,
    canceled: Vec<Source>,
    listener: Vec2,
}

impl AudioTracker {
    pub fn new(sources_limit: usize) -> Self {
        return Self {
            queue: Vec::new(),
            sources_limit,
            playing: Vec::new(),
            canceled: Vec::new(),
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

        let mut lowest: Option<(usize, bool, f32)> = None; // (index, is_playing, volume)

        for (i, queued) in self.queue.iter_mut().enumerate() {
            if audio.is_similar_to(queued) {
                queued.volume = f32::max(queued.volume, audio.volume);
                queued.duration = Duration::max(queued.duration, audio.duration);
                return;
            }

            if lowest.map_or(true, |(_, _, l)| queued.volume < l) {
                lowest = Some((i, false, queued.volume));
            }
        }

        if !self.has_space() {
            for (i, source) in self.playing.iter().enumerate() {
                if lowest.map_or(true, |(_, _, l)| source.volume < l) {
                    lowest = Some((i, true, source.volume));
                }
            }

            if let Some((i, is_playing, lowest)) = lowest {
                if lowest < audio.volume {
                    if is_playing {
                        self.canceled.push(self.playing.swap_remove(i));
                    } else {
                        self.queue.swap_remove(i);
                    }
                } else {
                    return;
                }
            } else {
                return;
            }
        }

        self.queue.push(audio);
    }

    pub fn update(
        &mut self,
        listener: Vec2,
        storage: &mut AudioStorage,
        audio: &Audio,
        sinks: &Assets<AudioSink>,
    ) {
        self.listener = listener;
        self.stop_expired(sinks);
        self.stop_canceled(sinks);
        self.play_queued(storage, audio, sinks);
    }

    fn stop_expired(&mut self, sinks: &Assets<AudioSink>) {
        let now = Instant::now();

        for i in (0..self.playing.len()).rev() {
            let source = &self.playing[i];

            if now > source.expiration {
                if source.force_stop {
                    if let Some(sink) = sinks.get(&source.handle) {
                        sink.stop();
                    }
                }

                self.playing.swap_remove(i);
            }
        }
    }

    fn stop_canceled(&mut self, sinks: &Assets<AudioSink>) {
        for canceled in self.canceled.drain(..) {
            if let Some(sink) = sinks.get(&canceled.handle) {
                sink.stop();
            }
        }
    }

    fn play_queued(
        &mut self,
        storage: &mut AudioStorage,
        audio: &Audio,
        sinks: &Assets<AudioSink>,
    ) {
        let mut queue = Vec::with_capacity(self.queue.capacity());
        std::mem::swap(&mut self.queue, &mut queue);

        for queued in queue.drain(..) {
            let audio_source = if let Some(handle) = storage.choose(queued.path.as_ref()) {
                handle.clone()
            } else {
                return;
            };

            let audio_sink = audio.play_with_settings(audio_source, queued.settings());
            let audio_sink_played = sinks.get_handle(audio_sink);
            let audio_duration_limit = queued.duration_limit();
            let audio_duration = audio_duration_limit.unwrap_or(AUDIO_DURATION);

            self.playing.push(Source {
                handle: audio_sink_played,
                expiration: Instant::now() + audio_duration + AUDIO_DURATION_EXTRA,
                volume: queued.volume,
                force_stop: audio_duration_limit.is_some(),
            });
        }
    }

    fn calc_spatial_volume(&self, source: Vec2, volume: f32) -> f32 {
        return f32::min(SOUND_DISTANCE_FACTOR / source.distance(self.listener), 1.0) * volume;
    }

    pub fn sources(&self) -> usize {
        return self.playing.len();
    }

    pub fn has_space(&self) -> bool {
        return self.playing.len() + self.queue.len() < self.sources_limit;
    }
}

struct Source {
    handle: Handle<AudioSink>,
    expiration: Instant,
    volume: f32,
    force_stop: bool,
}
