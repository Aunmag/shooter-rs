use crate::plugin::{camera_target::CameraTarget, AudioPlay, AudioStorage};
use bevy::{
    audio::{AudioPlayer, AudioSink, PlaybackMode, PlaybackSettings},
    ecs::{entity::Entity, world::World},
    prelude::{AudioSinkPlayback, Commands, Query, ResMut, Resource, Transform, Vec2, With},
    time::DelayedCommandsExt,
};
use std::sync::{
    atomic::{AtomicI16, Ordering},
    Mutex,
};

#[derive(Resource)]
pub struct AudioPool {
    inner: Mutex<AudioPoolInner>,
    limit: usize,
    score_threshold: AtomicI16,
    listener: Vec2,
}

struct AudioPoolInner {
    pool: Vec<PoolEntry>,
    to_stop: Vec<Entity>,
}

impl AudioPool {
    pub fn new(limit: usize) -> Self {
        return Self {
            inner: Mutex::new(AudioPoolInner {
                pool: Vec::with_capacity(limit),
                to_stop: Vec::with_capacity(limit),
            }),
            limit,
            score_threshold: AtomicI16::new(default_score_threshold(limit)),
            listener: Vec2::ZERO,
        };
    }

    pub fn queue(&self, mut audio: AudioPlay) {
        crate::util::bench::bench!();

        if let Some(source) = audio.source {
            audio.volume = audio.calc_spatial_volume(audio.volume, source, self.listener);
        }

        if audio.volume.is_nan() || audio.volume.is_infinite() {
            if cfg!(debug_assertions) {
                panic!(
                    "Got unexpected volume {} for sound {}",
                    audio.volume,
                    audio.path.as_ref(),
                );
            }

            return;
        }

        if audio.volume < AudioPlay::VOLUME_MIN {
            return;
        }

        let score = calc_score(audio.volume, audio.is_looped());

        if score <= self.score_threshold.load(Ordering::Relaxed) {
            return;
        }

        let Ok(mut inner) = self.inner.lock() else {
            log::error!("Unable to queue audio. Audio pool is poisoned"); // TODO: log once
            return;
        };

        // double check since atomic might be updated while mutex acquiring
        if score <= self.score_threshold.load(Ordering::Relaxed) {
            return;
        }

        if inner.pool.len() < self.limit {
            self.push(&mut inner.pool, PoolEntry::Pending(audio));
            return;
        }

        let mut lowest_score = None;

        for (i, other) in inner.pool.iter().enumerate() {
            let other_score = other.score();

            if other_score < score && lowest_score.is_none_or(|(_, s)| other_score < s) {
                lowest_score = Some((i, other_score));
            }
        }

        if let Some((i, lowest_score)) = lowest_score {
            if lowest_score < score {
                if let PoolEntry::Playing { entity, .. } = inner.pool[i] {
                    inner.to_stop.push(entity);
                }

                inner.pool[i] = PoolEntry::Pending(audio);
                self.update_score_threshold(&inner.pool);
            }
        }
    }

    fn push(&self, pool: &mut Vec<PoolEntry>, entry: PoolEntry) {
        pool.push(entry);

        debug_assert!(
            pool.len() <= self.limit,
            "audio pool overflow: {} > {}",
            pool.len(),
            self.limit,
        );

        if self.limit <= pool.len() {
            self.update_score_threshold(pool);
        }
    }

    fn update_score_threshold(&self, pool: &[PoolEntry]) {
        let mut threshold;

        if self.limit == 0 {
            threshold = i16::MAX;
        } else if pool.len() < self.limit {
            threshold = i16::MIN;
        } else {
            threshold = i16::MAX;

            for audio in pool {
                let score = audio.score();

                if threshold > score {
                    threshold = score;
                }
            }
        }

        self.score_threshold.store(threshold, Ordering::Relaxed);
    }

    pub fn listener(&self) -> Vec2 {
        return self.listener;
    }
}

pub(super) fn on_update(
    mut pool: ResMut<AudioPool>,
    mut storage: ResMut<AudioStorage>,
    mut commands: Commands,
    audio: Query<(Entity, &AudioSink, &PlaybackSettings)>,
    listeners: Query<&Transform, With<CameraTarget>>,
) {
    crate::util::bench::bench!();

    // ---------------------------------------------------------------------------------------------
    // update listener position

    if let Some(listener) = listeners.iter().next() {
        pool.listener = listener.translation.truncate();
    }

    // ---------------------------------------------------------------------------------------------
    // lock pool and reset caches

    let mut inner = pool.inner.lock().unwrap();
    pool.score_threshold
        .store(default_score_threshold(pool.limit), Ordering::Relaxed);

    // ---------------------------------------------------------------------------------------------
    // update playing sounds

    let mut playing = Vec::with_capacity(pool.limit);

    for (entity, sink, settings) in audio.iter() {
        if inner.to_stop.contains(&entity) {
            continue;
        }

        if sink.empty() {
            inner.to_stop.push(entity);
            continue;
        }

        let score = calc_score(
            sink.volume().to_linear(),
            matches!(settings.mode, PlaybackMode::Loop),
        );
        pool.push(&mut playing, PoolEntry::Playing { entity, score });
    }

    // ---------------------------------------------------------------------------------------------
    // remove stopped sounds

    if !inner.to_stop.is_empty() {
        let capacity = inner.to_stop.len();
        let to_stop = std::mem::replace(&mut inner.to_stop, Vec::with_capacity(capacity));

        commands.queue(move |world: &mut World| {
            for entity in to_stop {
                let _ = world.try_despawn(entity); // it might be already removed
            }
        });
    }

    // ---------------------------------------------------------------------------------------------
    // play pending sounds

    for audio in inner.pool.iter() {
        let PoolEntry::Pending(audio) = audio else {
            continue;
        };

        if let Some(entity) = play(audio, &mut storage, &mut commands) {
            let score = calc_score(audio.volume, audio.is_looped());
            pool.push(&mut playing, PoolEntry::Playing { entity, score });
        }
    }

    // ---------------------------------------------------------------------------------------------
    // finalize

    inner.pool = playing;
}

fn play(audio: &AudioPlay, storage: &mut AudioStorage, commands: &mut Commands) -> Option<Entity> {
    let source = storage.choose(audio.path.as_ref())?;
    let entity = commands.spawn((AudioPlayer(source), audio.settings())).id();

    if let Some(duration) = audio.duration() {
        commands
            .delayed()
            .duration(duration)
            .queue(move |w: &mut World| {
                let _ = w.try_despawn(entity); // it might be already removed
            });
    }

    return Some(entity);
}

enum PoolEntry {
    Pending(AudioPlay),
    Playing { entity: Entity, score: i16 },
}

impl PoolEntry {
    fn score(&self) -> i16 {
        match self {
            Self::Pending(audio) => {
                return calc_score(audio.volume, audio.is_looped());
            }
            Self::Playing { score, .. } => {
                return *score;
            }
        }
    }
}

const fn default_score_threshold(limit: usize) -> i16 {
    if limit == 0 {
        return i16::MAX;
    } else {
        return i16::MIN;
    }
}

// TODO: decrease score for long played sounds
const fn calc_score(volume: f32, is_looped: bool) -> i16 {
    if is_looped {
        // TODO: only if looped forever
        return i16::MAX;
    } else {
        return (volume * 1000.0) as i16;
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "allow unwraps in unit-tests")]
mod tests {
    use super::*;
    use crate::plugin::AudioPlay;
    use rand::seq::SliceRandom;

    #[test]
    fn queue() {
        let pool = AudioPool::new(4);
        assert_eq!(pool.debug(), ["-32768 --- threshold"]);

        assert_eq!(pool.test_push(10), ["-32768 --- threshold", "10 playing"]);

        assert_eq!(
            pool.test_push(40),
            ["-32768 --- threshold", "10 playing", "40 playing"],
        );

        assert_eq!(
            pool.test_queue(20),
            [
                "-32768 --- threshold",
                "10 playing",
                "20 pending",
                "40 playing",
            ],
        );

        assert_eq!(
            pool.test_queue(30),
            [
                "10 --- threshold",
                "10 playing",
                "20 pending",
                "30 pending",
                "40 playing",
            ],
        );

        assert_eq!(
            pool.test_queue(10),
            [
                "10 --- threshold",
                "10 playing",
                "20 pending",
                "30 pending",
                "40 playing",
            ],
        );

        assert_eq!(
            pool.test_queue(11),
            [
                "10 stopped",
                "11 --- threshold",
                "11 pending",
                "20 pending",
                "30 pending",
                "40 playing",
            ],
        );

        assert_eq!(
            pool.test_queue(35),
            [
                "10 stopped",
                "20 --- threshold",
                "20 pending",
                "30 pending",
                "35 pending",
                "40 playing",
            ],
        );
    }

    impl AudioPool {
        fn test_push(&self, score: i16) -> Vec<String> {
            self.shuffle(); // for fuzzy testing
            self.push(
                &mut self.inner.lock().unwrap().pool,
                PoolEntry::Playing {
                    entity: Entity::from_raw_u32(score as u32).unwrap(),
                    score,
                },
            );
            return self.debug();
        }

        fn test_queue(&self, score: i16) -> Vec<String> {
            self.shuffle(); // for fuzzy testing
            self.queue(AudioPlay {
                path: score.to_string().into(),
                volume: score as f32 / 1000.0,
                ..AudioPlay::DEFAULT
            });
            return self.debug();
        }

        fn shuffle(&self) {
            let mut rng = rand::rng();
            let mut inner = self.inner.lock().unwrap();
            inner.pool.shuffle(&mut rng);
            inner.to_stop.shuffle(&mut rng);
        }

        fn debug(&self) -> Vec<String> {
            let mut debug = Vec::new();
            let inner = self.inner.lock().unwrap();

            for audio in &inner.pool {
                match audio {
                    PoolEntry::Pending(audio) => {
                        debug.push(format!("{} pending", audio.path.as_ref()));
                    }
                    PoolEntry::Playing { entity, .. } => {
                        debug.push(format!("{} playing", entity.index_u32()));
                    }
                }
            }

            for entity in &inner.to_stop {
                debug.push(format!("{} stopped", entity.index_u32()));
            }

            debug.push(format!(
                "{} --- threshold",
                self.score_threshold.load(Ordering::Relaxed),
            ));

            debug.sort();

            return debug;
        }
    }
}
