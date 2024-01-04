use crate::{
    component::{Actor, Bot},
    util::ext::RngExt,
};
use bevy::{
    ecs::query::BatchingStrategy,
    math::Vec3Swizzles,
    prelude::{Entity, Query, Res, Transform},
    time::Time,
};
use std::time::Duration;

const TEAMMATES_MAX: usize = 8;
const UPDATE_INTERVAL: Duration = Duration::from_millis(1500);

pub fn analyze(
    mut bots: Query<(&mut Bot, Entity, &Actor, &Transform)>,
    actors: Query<(Entity, &Actor, &Transform)>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    bots.par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(32))
        .for_each(|(mut bot, e1, a1, t1)| {
            if !bot.update_timer.is_ready_or_disabled(time) {
                return;
            }

            let update_interval_fuzzed = bot.rng.fuzz_duration(UPDATE_INTERVAL);
            bot.update_timer.set(time + update_interval_fuzzed);
            bot.update_idle();
            bot.enemy = None;

            let p1 = t1.translation.xy();

            let mut enemy_distance = f32::MAX;
            let mut teammates = Teammates::new();

            for (e2, a2, t2) in actors.iter() {
                if e1 == e2 {
                    continue;
                }

                let distance = p1.distance_squared(t2.translation.truncate());

                if a1.config.kind == a2.config.kind {
                    let distance_max = bot.spread * 2.0;

                    if distance < distance_max * distance_max {
                        teammates.try_add(e2, distance);
                    }
                } else if distance < enemy_distance {
                    bot.enemy = Some(e2);
                    enemy_distance = distance;
                }
            }

            bot.teammates = teammates.teammates;
        });
}

struct Teammates {
    teammates: Vec<Entity>,
    distances: Vec<f32>,
    furthest_i: usize,
}

impl Teammates {
    fn new() -> Self {
        return Self {
            teammates: Vec::with_capacity(TEAMMATES_MAX),
            distances: Vec::with_capacity(TEAMMATES_MAX),
            furthest_i: 0,
        };
    }

    fn try_add(&mut self, teammate: Entity, distance: f32) {
        let distance_max = self.distances.get(self.furthest_i).copied().unwrap_or(0.0);

        if self.teammates.len() < TEAMMATES_MAX {
            self.teammates.push(teammate);
            self.distances.push(distance);

            if distance > distance_max {
                self.furthest_i = self.teammates.len() - 1;
            }
        } else if distance < distance_max {
            self.teammates[self.furthest_i] = teammate;
            self.distances[self.furthest_i] = distance;
            self.update_furthest();
        }
    }

    fn update_furthest(&mut self) {
        let mut distance_max = 0.0;

        for (i, distance) in self.distances.iter().enumerate() {
            if *distance > distance_max {
                self.furthest_i = i;
                distance_max = *distance;
            }
        }
    }
}
