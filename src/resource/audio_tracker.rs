use bevy::prelude::{Assets, AudioSink, AudioSinkPlayback, Handle, Resource};
use derive_more::Constructor;
use std::time::{Duration, Instant};

/// Just in case to prevent clipping
const EXTRA_DURATION: Duration = Duration::from_millis(50);

#[derive(Resource)]
pub struct AudioTracker {
    sources_limit: usize,
    sources: Vec<Source>,
}

impl AudioTracker {
    pub fn new(sources_limit: usize) -> Self {
        return Self {
            sources_limit,
            sources: Vec::new(),
        };
    }

    pub fn update(&mut self, sinks: &Assets<AudioSink>) {
        let now = Instant::now();

        for i in (0..self.sources.len()).rev() {
            let source = &self.sources[i];

            if now > source.expiration {
                if source.force_stop {
                    if let Some(sink) = sinks.get(&source.handle) {
                        sink.stop();
                    }
                }

                self.sources.swap_remove(i);
            }
        }
    }

    pub fn provide_space(
        &mut self,
        priority: u8,
        volume: f32,
    ) -> (bool, Option<Handle<AudioSink>>) {
        if self.has_space() {
            return (true, None);
        }

        let mut lowest: Option<(usize, &Source)> = None;

        for (i, source) in self.sources.iter().enumerate() {
            if lowest.map_or(true, |(_, l)| source.priority.is_lower_than(l.priority)) {
                lowest = Some((i, source));
            }
        }

        if let Some((i, lowest)) = lowest {
            if lowest
                .priority
                .is_lower_than(Priority::new(priority, volume))
            {
                return (true, Some(self.sources.swap_remove(i).handle));
            }
        }

        return (false, None);
    }

    pub fn register(
        &mut self,
        handle: Handle<AudioSink>,
        priority: u8,
        volume: f32,
        duration: Duration,
        force_stop: bool,
    ) {
        debug_assert!(self.has_space());

        self.sources.push(Source {
            handle,
            expiration: Instant::now() + duration + EXTRA_DURATION,
            priority: Priority::new(priority, volume),
            force_stop,
        })
    }

    pub fn sources(&self) -> usize {
        return self.sources.len();
    }

    pub fn has_space(&self) -> bool {
        return self.sources.len() < self.sources_limit;
    }
}

struct Source {
    handle: Handle<AudioSink>,
    expiration: Instant,
    priority: Priority,
    force_stop: bool,
}

#[derive(Clone, Copy, Constructor)]
struct Priority {
    priority: u8,
    volume: f32,
}

impl Priority {
    fn is_lower_than(self, other: Self) -> bool {
        return self.priority < other.priority
            || (self.priority == other.priority && self.volume < other.volume);
    }
}
