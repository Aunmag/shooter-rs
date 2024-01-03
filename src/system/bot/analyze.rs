use crate::{
    component::{Actor, Bot, Inertia},
    util::math::find_meet_point,
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
    mut bots: Query<(&mut Bot, Entity, &Actor, &Transform, &Inertia)>,
    actors: Query<(Entity, &Actor, &Transform, &Inertia)>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    bots.par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(32))
        .for_each(|(mut bot, e1, a1, t1, i1)| {
            if !bot.update_timer.next_if_ready(time, || UPDATE_INTERVAL) {
                return;
            }

            bot.enemy = None;

            let p1 = t1.translation.xy();

            let mut teammates = Vec::<(Entity, f32)>::with_capacity(TEAMMATES_MAX);
            let mut enemy_distance = f32::MAX;

            for (e2, a2, t2, i2) in actors.iter() {
                if e1 == e2 {
                    continue;
                }

                let p2 = t2.translation.xy();

                if a1.config.kind == a2.config.kind {
                    let distance = (p1 - p2).length_squared();
                    let distance_max = bot.spread * 2.0;

                    if distance < distance_max * distance_max {
                        if teammates.len() < TEAMMATES_MAX {
                            teammates.push((e2, distance));
                        } else {
                            replace_farthest(&mut teammates, e2, distance);
                        }
                    }
                } else {
                    let position_meet = find_meet_point(p1, i1.velocity.length(), p2, i2.velocity);
                    let distance = (p1 - position_meet).length_squared();

                    if distance < enemy_distance {
                        bot.enemy = Some(e2);
                        enemy_distance = distance;
                    }
                }
            }

            bot.teammates = teammates.iter().map(|t| t.0).collect();
        });
}

fn replace_farthest(teammates: &mut [(Entity, f32)], entity: Entity, distance_new: f32) {
    let mut farthest: Option<(usize, f32)> = None;

    for (i, (_, distance)) in teammates.iter().enumerate() {
        if *distance > distance_new && farthest.map_or(true, |f| *distance > f.1) {
            farthest = Some((i, *distance));
        }
    }

    if let Some((i, _)) = farthest {
        teammates[i] = (entity, distance_new);
    }
}
