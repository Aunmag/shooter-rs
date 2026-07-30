mod audio_play;
mod audio_storage;

pub use self::{audio_play::*, audio_storage::*};
use crate::plugin::{camera_target::CameraTarget, Heartbeat};
use bevy::{
    app::Update,
    audio::{AudioPlayer, AudioSink, Volume},
    ecs::{component::Component, entity::Entity},
    prelude::{
        App, AudioSinkPlayback, Commands, Plugin, Query, Res, ResMut, Resource, Time, Transform,
        Vec2, With,
    },
};
use std::{sync::Mutex, time::Duration};

pub struct AudioPlugin {
    limit: usize,
}

impl AudioPlugin {
    pub fn new(limit: usize) -> Self {
        return Self { limit };
    }
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioStorage::default());
        app.insert_resource(AudioTracker {
            queue: Mutex::new(Vec::with_capacity(self.limit)),
            playing: 0,
            limit: self.limit,
            listener: Vec2::ZERO,
        });

        app.add_systems(Update, on_update);
    }
}

// TODO: find a better name
#[derive(Resource)]
pub struct AudioTracker {
    queue: Mutex<Vec<AudioPlay>>,
    limit: usize, // TODO: autoupdate from settings
    pub playing: usize,
    pub listener: Vec2,
}

impl AudioTracker {
    // TODO: ability to queue with command?
    pub fn queue(&self, mut audio: AudioPlay) {
        crate::util::bench::bench!();

        if let Some(source) = audio.source {
            audio.volume = audio.calc_spatial_volume(audio.volume, source, self.listener);
        }

        if audio.volume < AudioPlay::VOLUME_MIN {
            return;
        }

        let Ok(mut queue) = self.queue.lock() else {
            log::error!("Unable to queue audio. Audio tracker is poisoned");
            return;
        };

        let is_overflow = self.playing + queue.len() >= self.limit;
        let mut replacement = None;

        // TODO: to separate method
        for (i, other) in queue.iter().enumerate() {
            if audio.is_similar_to(other) {
                return;
            }

            if is_overflow
                && other.volume < audio.volume
                && replacement.is_none_or(|(_, v)| other.volume > v)
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

    fn take_queue(&self) -> Vec<AudioPlay> {
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
}

#[derive(Component)]
struct Expiration(Duration);

fn on_update(
    mut tracker: ResMut<AudioTracker>,
    mut storage: ResMut<AudioStorage>,
    mut commands: Commands,
    audio: Query<(Entity, &AudioSink, Option<&Expiration>)>,
    listeners: Query<&Transform, With<CameraTarget>>,
    time: Res<Time>,
) {
    crate::util::bench::bench!();
    let now = time.elapsed();

    if let Some(listener) = listeners.iter().next() {
        tracker.listener = listener.translation.truncate();
    }

    tracker.playing = 0;

    for (entity, sink, expiration) in audio.iter() {
        if sink.empty() || expiration.is_some_and(|e| now > e.0) {
            sink.stop();
            commands.entity(entity).despawn();
        } else {
            tracker.playing += 1;
        }
    }

    for audio in &tracker.take_queue() {
        let Some(source) = storage.choose(audio.path.as_ref()) else {
            continue;
        };

        let is_heartbeat = audio.path.as_ref() == Heartbeat::PATH;
        let mut settings = audio.settings();

        if is_heartbeat {
            settings.volume = Volume::Linear(0.0);
        }

        let mut entity = commands.spawn((AudioPlayer(source), settings));

        if is_heartbeat {
            entity.insert(Heartbeat);
        }

        if let Some(duration) = audio.duration() {
            entity.insert(Expiration(now + duration));
        }

        tracker.playing += 1;
    }
}
