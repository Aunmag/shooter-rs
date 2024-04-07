use crate::{
    component::AudioExpiration,
    plugin::{CameraTarget, Heartbeat},
    resource::{AudioStorage, AudioTracker},
};
use bevy::{
    audio::{AudioBundle, AudioSink, Volume, VolumeLevel},
    ecs::{entity::Entity, system::Commands},
    prelude::{AudioSinkPlayback, DespawnRecursiveExt, Query, Res, ResMut, Transform, With},
    time::Time,
};

pub fn audio(
    mut tracker: ResMut<AudioTracker>,
    mut storage: ResMut<AudioStorage>,
    mut commands: Commands,
    audio: Query<(Entity, &AudioSink, Option<&AudioExpiration>)>,
    listeners: Query<&Transform, With<CameraTarget>>,
    time: Res<Time>,
) {
    let now = time.elapsed();

    if let Some(listener) = listeners.iter().next() {
        tracker.listener = listener.translation.truncate();
    }

    tracker.playing = 0;

    for (entity, sink, expiration) in audio.iter() {
        if sink.empty() || expiration.map_or(false, |e| now > **e) {
            sink.stop();
            commands.entity(entity).despawn_recursive();
        } else {
            tracker.playing += 1;
        }
    }

    tracker.update_delayed(now);

    for audio in &tracker.take_queue() {
        let Some(source) = storage.choose(audio.path.as_ref()) else {
            continue;
        };

        let is_heartbeat = audio.path.as_ref() == Heartbeat::PATH;
        let mut settings = audio.settings();

        if is_heartbeat {
            settings.volume = Volume::Relative(VolumeLevel::new(0.0));
        }

        let mut entity = commands.spawn(AudioBundle { source, settings });

        if is_heartbeat {
            entity.insert(Heartbeat);
        }

        if let Some(duration) = audio.duration() {
            entity.insert(AudioExpiration::new(now + duration));
        }

        tracker.playing += 1;
    }
}
