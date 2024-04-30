use crate::{
    model::AudioPlay,
    plugin::{camera_target::CameraTarget, Heartbeat},
    resource::AudioStorage,
};
use bevy::{
    app::Update,
    audio::{AudioBundle, AudioSink, Volume, VolumeLevel},
    ecs::{component::Component, entity::Entity},
    prelude::{
        App, AudioSinkPlayback, Commands, DespawnRecursiveExt, Plugin, Query, Res, ResMut,
        Resource, Time, Transform, Vec2, With,
    },
};
use std::{sync::Mutex, time::Duration};

const VOLUME_MIN: f32 = 0.01;
const SOUND_DISTANCE_FACTOR: f32 = 2.0;

pub struct AudioTrackerPlugin;

impl Plugin for AudioTrackerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_update);
    }
}

#[derive(Resource)]
pub struct AudioTracker {
    queue: Mutex<Vec<AudioPlay>>,
    queue_delayed: Mutex<Vec<(AudioPlay, Duration)>>,
    limit: usize,
    pub playing: usize,
    pub listener: Vec2,
}

impl AudioTracker {
    pub fn new(sources_limit: usize) -> Self {
        return Self {
            queue: Mutex::new(Vec::with_capacity(sources_limit)),
            queue_delayed: Mutex::new(Vec::new()),
            playing: 0,
            limit: sources_limit,
            listener: Vec2::ZERO,
        };
    }

    pub fn queue(&self, mut audio: AudioPlay) {
        if let Some(source) = audio.source {
            audio.volume = self.calc_spatial_volume(source, audio.volume);
        }

        if audio.volume < VOLUME_MIN {
            return;
        }

        let Ok(mut queue) = self.queue.lock() else {
            log::error!("Unable to queue audio. Audio tracker is poisoned");
            return;
        };

        let is_overflow = self.playing + queue.len() >= self.limit;
        let mut replacement = None;

        for (i, other) in queue.iter().enumerate() {
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
                queue[i] = audio;
            }
        } else {
            queue.push(audio);
        }
    }

    pub fn queue_delayed(&self, time: Duration, audio: AudioPlay) {
        if let Ok(mut queue) = self.queue_delayed.lock() {
            queue.push((audio, time));
        }
    }

    fn update_delayed(&self, now: Duration) {
        let Ok(mut queue_delayed) = self.queue_delayed.lock() else {
            return;
        };

        for i in (0..queue_delayed.len()).rev() {
            if queue_delayed[i].1 <= now {
                self.queue(queue_delayed.swap_remove(i).0);
            }
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

    pub fn calc_spatial_volume(&self, source: Vec2, volume: f32) -> f32 {
        return f32::min(SOUND_DISTANCE_FACTOR / source.distance(self.listener), 1.0) * volume;
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
    let now = time.elapsed();

    if let Some(listener) = listeners.iter().next() {
        tracker.listener = listener.translation.truncate();
    }

    tracker.playing = 0;

    for (entity, sink, expiration) in audio.iter() {
        if sink.empty() || expiration.map_or(false, |e| now > e.0) {
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
            entity.insert(Expiration(now + duration));
        }

        tracker.playing += 1;
    }
}
