use crate::{
    data::DISTANCE_1MM,
    plugin::{
        debug::{debug_circle, debug_line},
        kinetics::{self, Kinetics},
    },
    state::AppState,
    util::ext::{AppExt, Vec2Ext},
};
use bevy::{
    color::{
        palettes::css::{RED, WHITE, YELLOW},
        Alpha, Srgba,
    },
    ecs::{component::Component, schedule::SystemSet, system::Local},
    math::Vec2,
    platform::collections::HashMap,
    prelude::{App, Entity, IntoScheduleConfigs, Plugin, Query, Transform},
};
use rand::RngExt;
use std::f32::consts::TAU;

const DEBUG: bool = false;
const EXTRA_RESOLVE_DISTANCE: f32 = DISTANCE_1MM;

pub struct CollisionPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CollisionSystems;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(
            AppState::Game,
            on_update
                .in_set(CollisionSystems)
                .after(kinetics::on_update),
        );
    }
}

#[derive(Component, Clone, Copy)]
pub struct Collision {
    pub radius: f32,
}

fn on_update(
    mut query: Query<(Entity, &mut Transform, &Collision, &mut Kinetics)>,
    mut cache: Local<SpatialIndex>,
) {
    crate::util::bench::bench!();

    for (entity, transform, collision, _) in query.iter() {
        let position = transform.translation.truncate();

        if DEBUG {
            debug_circle(position, collision.radius, WHITE);
        }

        cache.insert(position, (entity, position, *collision));
    }

    if DEBUG {
        for chunk_id in cache.chunks.keys() {
            chunk_id.draw_debug(WHITE);
        }
    }

    while let Some((e1, p1, c1)) = cache.pop() {
        cache.iter_neighbors(p1, |(e2, p2, c2)| {
            let distance = p2 - p1;
            let distance_min = c1.radius + c2.radius;

            if distance.is_short(distance_min) {
                let angle = if distance.is_zero() {
                    Vec2::from_angle(rand::rng().random_range(0.0..TAU))
                } else {
                    distance.normalize()
                };

                let shift_distance =
                    (distance_min - distance.length()) / 2.0 + EXTRA_RESOLVE_DISTANCE;

                let push = if let (Ok(e1), Ok(e2)) = (
                    query.get(e1),  // TODO: just get single component?
                    query.get(*e2), // TODO: just get single component?
                ) {
                    Kinetics::bounce(e1.3, e2.3, angle)
                } else {
                    Vec2::ZERO
                };

                if DEBUG {
                    debug_circle(p1, c1.radius, RED);
                    debug_circle(*p2, c2.radius, RED);
                }

                let shift = angle * shift_distance;

                if let Ok(mut e) = query.get_mut(e1) {
                    e.1.translation.x -= shift.x;
                    e.1.translation.y -= shift.y;
                    e.3.push(push, 0.0, false);
                }

                if let Ok(mut e) = query.get_mut(*e2) {
                    e.1.translation.x += shift.x;
                    e.1.translation.y += shift.y;
                    e.3.push(-push, 0.0, false);
                }
            }
        });
    }
}
#[derive(Default)]
struct SpatialIndex {
    chunks: HashMap<SpatialId, Vec<SpatialData>>,
}

impl SpatialIndex {
    fn insert(&mut self, position: Vec2, value: SpatialData) {
        self.chunks
            .entry(SpatialId::from(position))
            .or_insert_with(|| Vec::with_capacity(8))
            .push(value);
    }

    fn pop(&mut self) -> Option<SpatialData> {
        let mut value = None;
        let mut empty_chunk_id = None;

        for (id, values) in self.chunks.iter_mut() {
            value = values.pop();

            if value.is_none() || values.is_empty() {
                empty_chunk_id = Some(*id);
            }

            if value.is_some() {
                break;
            }
        }

        if let Some(id) = empty_chunk_id {
            self.chunks.remove(&id);
        }

        return value;
    }

    fn iter_neighbors<F: FnMut(&SpatialData)>(&self, position: Vec2, mut f: F) {
        let origin = SpatialId::from(position);
        let offset_direction = SpatialId {
            x: calc_offset_direction(position.x),
            y: calc_offset_direction(position.y),
        };

        for offset in [(0, 0), (1, 0), (0, 1), (1, 1)] {
            let id = SpatialId {
                x: origin.x + offset.0 * offset_direction.x,
                y: origin.y + offset.1 * offset_direction.y,
            };

            if DEBUG && offset != (0, 0) {
                id.draw_debug(YELLOW.with_alpha(0.3));
            }

            self.iter_chunk(&id, &mut f);
        }
    }

    fn iter_chunk<F: FnMut(&SpatialData)>(&self, id: &SpatialId, f: &mut F) {
        if let Some(chunk) = self.chunks.get(id) {
            for item in chunk.iter() {
                f(item);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct SpatialId {
    x: i32,
    y: i32,
}

impl SpatialId {
    fn draw_debug(&self, color: Srgba) {
        let v = |x: i32, y: i32| Vec2::new(x as f32, y as f32);
        let p = v(self.x, self.y);
        debug_line(p + v(0, 0), p + v(1, 0), color);
        debug_line(p + v(1, 0), p + v(1, 1), color);
        debug_line(p + v(1, 1), p + v(0, 1), color);
        debug_line(p + v(0, 1), p + v(0, 0), color);
    }
}

impl From<Vec2> for SpatialId {
    fn from(position: Vec2) -> Self {
        return Self {
            x: position.x.floor() as i32,
            y: position.y.floor() as i32,
        };
    }
}

type SpatialData = (Entity, Vec2, Collision);

fn calc_offset_direction(n: f32) -> i32 {
    let s = n.signum() as i32;

    if n.fract().abs() >= 0.5 {
        return s;
    } else {
        return -s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_offset_direction() {
        assert_eq!(calc_offset_direction(12.45), -1);
        assert_eq!(calc_offset_direction(12.55), 1);
        assert_eq!(calc_offset_direction(-12.45), 1);
        assert_eq!(calc_offset_direction(-12.55), -1);
    }
}
