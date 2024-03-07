use crate::{
    component::Actor,
    plugin::bot::Bot,
    util::{ext::Vec2Ext, math::angle_difference, Timer},
};
use bevy::{
    ecs::{
        entity::Entity,
        schedule::SystemConfigs,
        system::{Local, Res},
    },
    math::Vec2,
    prelude::{IntoSystemConfigs, Query, Transform, With},
    time::Time,
    utils::HashMap,
};
use rand::Rng;
use std::{
    f32::consts::{FRAC_PI_2, TAU},
    time::Duration,
};

const UPDATE_INTERVAL: Duration = Duration::from_secs(8);
const DIRECTIONS: usize = 6;
const DIRECTION_STEP: f32 = TAU / DIRECTIONS as f32;
const DISTANCE_MIN: f32 = 5.0;
const DISTANCE_MAX: f32 = 20.0;
const TURN_SMOOTH: f32 = 1.2;

pub struct Detour {
    pub angle: f32,
    pub distance: f32,
}

impl Detour {
    pub fn calc(&self, source: Vec2, target: Vec2) -> Option<f32> {
        let angle_from_target = target.angle_to(source);
        let angular_distance = angle_difference(angle_from_target, self.angle);

        if angular_distance.abs() < DIRECTION_STEP / 2.0 {
            // detour is reached, no need to detour anymore
            return None;
        } else if source.is_close(target, self.distance) {
            // target it quite close to start detour
            return Some(angle_from_target + FRAC_PI_2 * TURN_SMOOTH * angular_distance.signum());
        } else {
            // target is too far, go towards it
            return None;
        }
    }
}

pub fn on_update() -> SystemConfigs {
    return on_update_inner.run_if(|mut r: Local<Timer>, t: Res<Time>| {
        r.next_if_ready(t.elapsed(), || UPDATE_INTERVAL)
    });
}

fn on_update_inner(
    mut bots: Query<(Entity, &mut Bot, &Transform)>,
    actors: Query<&Transform, With<Actor>>,
) {
    let mut directions_by_target = HashMap::new();

    for (attacker, attacker_bot, attacker_transform) in bots.iter() {
        let Some(target) = attacker_bot.enemy else {
            continue;
        };

        let Ok(target_point) = actors.get(target).map(|t| t.translation.truncate()) else {
            continue;
        };

        let attacker_point = attacker_transform.translation.truncate();
        let attacker_angle = target_point.angle_to(attacker_point);

        directions_by_target
            .entry(target)
            .or_insert_with(Directions::new)
            .add(attacker, attacker_angle);
    }

    for (_target, directions) in directions_by_target.iter_mut() {
        directions.normalize();

        for direction in &directions.0 {
            for (attacker, _) in &direction.attackers {
                let Some(mut bot) = bots.get_mut(*attacker).ok() else {
                    continue;
                };

                bot.1.detour = Some(Detour {
                    angle: direction.angle,
                    distance: rand::thread_rng().gen_range(DISTANCE_MIN..DISTANCE_MAX),
                });
            }
        }
    }
}

struct Directions(Vec<Direction>);

impl Directions {
    fn new() -> Self {
        let mut directions = Vec::with_capacity(DIRECTIONS);

        for i in 0..DIRECTIONS {
            directions.push(Direction {
                angle: DIRECTION_STEP * i as f32,
                attackers: Vec::new(),
                is_visible: true,
            });
        }

        return Self(directions);
    }

    fn add(&mut self, entity: Entity, angle: f32) {
        if let Some(direction) = self
            .find_closest(angle, true)
            .and_then(|i| self.0.get_mut(i))
        {
            direction.attackers.push((entity, angle));
        }
    }

    fn normalize(&mut self) {
        let attackers_per_direction = self.count_entities() / self.0.len();

        while let Some(i) = self.find_smallest_crowd() {
            let direction = &mut self.0[i];
            let direction_angle = direction.angle;
            direction.is_visible = false;
            let missing_attackers =
                attackers_per_direction.saturating_sub(direction.attackers.len());

            if missing_attackers == 0 {
                break;
            }

            let angle_l = direction_angle + DIRECTION_STEP;
            let angle_r = direction_angle - DIRECTION_STEP;

            for _ in 0..missing_attackers {
                let Some(near_l) = self.find_closest(angle_l, false) else {
                    break;
                };

                let Some(near_r) = self.find_closest(angle_r, false) else {
                    break;
                };

                let near = if self.0[near_l].attackers.len() > self.0[near_r].attackers.len() {
                    near_l
                } else {
                    near_r
                };

                if let Some(attacker) = self.0[near].take_closest(direction_angle) {
                    self.0[i].attackers.push((attacker, 0.0));
                }
            }
        }
    }

    fn count_entities(&self) -> usize {
        let mut entities = 0;

        for direction in &self.0 {
            entities += direction.attackers.len();
        }

        return entities;
    }

    fn find_closest(&self, angle: f32, allow_empty: bool) -> Option<usize> {
        return self.find_closest_by(allow_empty, |d| angle_difference(d.angle, angle).abs());
    }

    fn find_smallest_crowd(&self) -> Option<usize> {
        return self.find_closest_by(true, |d| d.attackers.len() as f32);
    }

    fn find_closest_by<F: Fn(&Direction) -> f32>(&self, allow_empty: bool, f: F) -> Option<usize> {
        let mut best_i = None;
        let mut best_diff = f32::INFINITY;

        for (i, direction) in self.0.iter().enumerate() {
            if !direction.is_visible {
                continue;
            }

            if !allow_empty && direction.attackers.is_empty() {
                continue;
            }

            let diff = f(direction);

            if diff < best_diff {
                best_i = Some(i);
                best_diff = diff;
            }
        }

        return best_i;
    }
}

struct Direction {
    angle: f32,
    attackers: Vec<(Entity, f32)>,
    is_visible: bool,
}

impl Direction {
    fn take_closest(&mut self, desired_angle: f32) -> Option<Entity> {
        if self.attackers.is_empty() {
            return None;
        }

        let mut closest = 0;
        let mut closest_distance = f32::INFINITY;

        for (i, (_, attacker_angle)) in self.attackers.iter().enumerate() {
            let distance = angle_difference(*attacker_angle, desired_angle).abs();

            if distance < closest_distance {
                closest = i;
                closest_distance = distance;
            }
        }

        return Some(self.attackers.swap_remove(closest).0);
    }
}
