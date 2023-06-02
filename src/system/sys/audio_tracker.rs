use crate::resource::AudioTracker;
use bevy::{
    audio::AudioSink,
    prelude::{Assets, Res, ResMut},
};

pub fn audio_tracker(mut audio_tracker: ResMut<AudioTracker>, sinks: Res<Assets<AudioSink>>) {
    audio_tracker.update(&sinks);
}
