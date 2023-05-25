use crate::resource::AudioTracker;
use bevy::{
    audio::AudioSink,
    prelude::{Assets, Res, ResMut},
    time::Time,
};

pub fn audio_tracker(
    mut audio_tracker: ResMut<AudioTracker>,
    sinks: Res<Assets<AudioSink>>,
    time: Res<Time>,
) {
    audio_tracker.update(time.elapsed(), &sinks);
}
