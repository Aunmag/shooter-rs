use bevy::prelude::{Assets, AudioSink, AudioSinkPlayback, Handle, Resource};
use std::time::Duration;

#[derive(Default, Resource)]
pub struct AudioTracker {
    temporary: Vec<(Handle<AudioSink>, Duration)>,
}

impl AudioTracker {
    pub fn update(&mut self, time: Duration, sinks: &Assets<AudioSink>) {
        for i in (0..self.temporary.len()).rev() {
            let (handle, ttl) = &self.temporary[i];

            if *ttl < time {
                if let Some(sink) = sinks.get(handle) {
                    sink.stop();
                }

                self.temporary.swap_remove(i);
            }
        }
    }

    pub fn track_temporary(&mut self, handle: Handle<AudioSink>, ttl: Duration) {
        self.temporary.push((handle, ttl));
    }
}
