use crate::{
    component::{AudioExpiration, Heartbeat, Player},
    resource::{AudioStorage, AudioTracker},
};
use bevy::{
    audio::{AudioBundle, AudioSink, Volume, VolumeLevel},
    ecs::{entity::Entity, system::Commands},
    prelude::{AudioSinkPlayback, DespawnRecursiveExt, Query, Res, ResMut, Transform, With},
    time::Time,
};
use std::time::Duration;

pub fn audio(
    mut tracker: ResMut<AudioTracker>,
    mut storage: ResMut<AudioStorage>,
    mut commands: Commands,
    audio: Query<(Entity, &AudioSink, Option<&AudioExpiration>)>,
    listeners: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let now = time.elapsed();

    tracker.playing = 0;
    tracker.listener = listeners
        .iter()
        .next()
        .map(|l| l.translation.xy())
        .unwrap_or(Vec2::ZERO);

    for (entity, sink, expiration) in audio.iter() {
        if sink.empty() || expiration.map_or(false, |e| now > **e) {
            sink.stop();
            commands.entity(entity).despawn_recursive();
        } else {
            tracker.playing += 1;
        }
    }

    for queued in &tracker.take_queue() {
        if let Some(source) = storage.choose(queued.path.as_ref()).cloned() {
            let is_heartbeat = queued.path.as_ref() == "sounds/heartbeat";
            let mut settings = queued.settings();

            if is_heartbeat {
                settings.volume = Volume::Relative(VolumeLevel::new(0.0));
            }

            let mut entity = commands.spawn(AudioBundle { source, settings });

            if is_heartbeat {
                entity.insert(Heartbeat);
            }

            if Duration::ZERO < queued.duration && queued.duration < Duration::MAX {
                entity.insert(AudioExpiration::new(now + queued.duration));
            }

            tracker.playing += 1;
        }
    }
}
