use crate::{
    component::Player,
    resource::{AudioStorage, AudioTracker},
};
use bevy::{
    audio::AudioSink,
    math::Vec3Swizzles,
    prelude::{Assets, Audio, Query, Res, ResMut, Transform, Vec2, With},
};

pub fn audio_tracker(
    mut audio_tracker: ResMut<AudioTracker>,
    mut storage: ResMut<AudioStorage>,
    audio: Res<Audio>,
    sinks: Res<Assets<AudioSink>>,
    listeners: Query<&Transform, With<Player>>,
) {
    let listener = listeners
        .iter()
        .next()
        .map(|l| l.translation.xy())
        .unwrap_or(Vec2::ZERO);

    audio_tracker.update(listener, &mut storage, &audio, &sinks);
}
