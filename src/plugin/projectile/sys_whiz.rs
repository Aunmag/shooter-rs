use crate::{
    plugin::{
        camera_target::CameraTarget, debug::debug_line, projectile::state::ProjectileState,
        AudioPlay, AudioPool, Projectile, ProjectilePhysics,
    },
    util::geometry::GeometryProjection,
};
use bevy::{
    color::palettes::css::RED,
    ecs::{query::With, system::Query},
    prelude::{Res, Time, Transform},
};

const DEBUG: bool = false;

pub fn on_update(
    mut projectiles: Query<&Projectile>,
    listeners: Query<&Transform, With<CameraTarget>>,
    audio: Res<AudioPool>,
    time: Res<Time>,
) {
    // TODO: early return if sounds disabled

    crate::util::bench::bench!();
    let t0 = time.elapsed();
    let t1 = t0.saturating_sub(time.delta());

    for listener in listeners.iter() {
        let listener = listener.translation.truncate();
        let mut closest = None;
        let mut closest_distance = f32::INFINITY;

        for projectile in projectiles.iter_mut() {
            if projectile.config.physics != ProjectilePhysics::Bullet {
                continue;
            }

            let head = ProjectileState::calc(projectile, t0).position();
            let tail = ProjectileState::calc(projectile, t1).position();
            let length_squared = head.distance_squared(tail);

            if length_squared == 0.0 {
                continue;
            }

            let projection = listener.project_on(&(head, tail));

            if DEBUG {
                debug_line(listener, projection, RED);
            }

            if projection.distance_squared(head) > length_squared {
                continue;
            }

            if projection.distance_squared(tail) > length_squared {
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
                path: "sounds/bullet/air".into(),
                volume: 0.8,
                falloff: AudioPlay::FALLOFF_SHORTEST,
                source: Some(source),
                ..AudioPlay::DEFAULT
            });
        }
    }
}
