use crate::{
    model::{geometry::GeometryProjection, AudioPlay},
    plugin::{debug::debug_line, AudioTracker, CameraTarget, Projectile},
};
use bevy::{
    ecs::{query::With, system::Query},
    prelude::{Res, Time, Transform},
    render::color::Color,
};

const DEBUG: bool = false;

pub fn on_update(
    mut projectiles: Query<&Projectile>,
    listeners: Query<&Transform, With<CameraTarget>>,
    audio: Res<AudioTracker>,
    time: Res<Time>,
) {
    let t0 = time.elapsed();
    let t1 = t0.saturating_sub(time.delta());

    for listener in listeners.iter() {
        let listener = listener.translation.truncate();
        let mut closest = None;
        let mut closest_distance = f32::INFINITY;

        for projectile in projectiles.iter_mut() {
            let head = projectile.calc_data(t0).0;
            let tail = projectile.calc_data(t1).0;
            let length = head.distance_squared(tail);

            let projection = listener.project_on(&(head, tail));

            if DEBUG {
                debug_line(listener, projection, Color::RED);
            }

            let to_head_distance = projection.distance_squared(head);

            if to_head_distance > length {
                continue;
            }

            let to_tail_distance = projection.distance_squared(tail);

            if to_tail_distance > length {
                continue;
            }

            let distance = listener.distance_squared(projection);

            if distance < closest_distance {
                closest = Some(projection);
                closest_distance = distance;
            }
        }

        if let Some(source) = closest {
            audio.queue(AudioPlay {
                path: "sounds/bullet_whiz".into(),
                volume: 0.8,
                source: Some(source),
                ..AudioPlay::DEFAULT
            });
        }
    }
}
