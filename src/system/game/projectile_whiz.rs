use crate::{
    component::{Player, Projectile},
    model::{geometry::GeometryProjection, AudioPlay},
    resource::AudioTracker,
    util::GIZMOS,
};
use bevy::{
    ecs::{query::With, system::Query},
    prelude::{Res, Time, Transform},
    render::color::Color,
};

const DEBUG: bool = false;

pub fn projectile_whiz(
    mut projectiles: Query<&Projectile>,
    players: Query<&Transform, With<Player>>,
    audio: Res<AudioTracker>,
    time: Res<Time>,
) {
    let t0 = time.elapsed();
    let t1 = t0.saturating_sub(time.delta());

    for player in players.iter() {
        let player_position = player.translation.truncate();

        for projectile in projectiles.iter_mut() {
            let head = projectile.calc_data(t0).0;
            let tail = projectile.calc_data(t1).0;
            let length = head.distance_squared(tail);

            let projection = player_position.project_on(&(head, tail));

            if DEBUG {
                GIZMOS.ln(player_position, projection, Color::RED);
            }

            let to_head_distance = projection.distance_squared(head);

            if to_head_distance > length {
                continue;
            }

            let to_tail_distance = projection.distance_squared(tail);

            if to_tail_distance > length {
                continue;
            }

            audio.queue(AudioPlay {
                path: "sounds/bullet_whiz".into(),
                volume: 0.8,
                source: Some(projection),
                ..AudioPlay::DEFAULT
            });
        }
    }
}
