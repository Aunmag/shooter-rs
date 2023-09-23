use crate::{
    component::{Collision, Projectile},
    model::{geometry::GeometryProjection, AudioPlay},
    resource::{AudioTracker, HitResource},
    util::{ext::Vec2Ext, math},
};
use bevy::{
    ecs::{entity::Entity, system::Query},
    math::{Quat, Vec3Swizzles},
    prelude::{Commands, DespawnRecursiveExt, Res, ResMut, Time, Transform, Vec2, Without},
};
use std::time::Duration;

const TIME_DELTA_FOR_RENDER: Duration = Duration::from_millis(25); // 40 FPS

pub fn projectile(
    mut projectiles: Query<(Entity, &mut Projectile, &mut Transform)>,
    obstacles: Query<(Entity, &Collision, &Transform), Without<Projectile>>,
    mut hits: ResMut<HitResource>,
    mut commands: Commands,
    mut audio: ResMut<AudioTracker>,
    time: Res<Time>,
) {
    let t0 = time.elapsed();
    let t1 = t0.saturating_sub(time.delta());
    let t2 = t0.saturating_sub(Duration::max(time.delta(), TIME_DELTA_FOR_RENDER));

    for (entity, mut projectile, mut transform) in projectiles.iter_mut() {
        if projectile.stopped {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        let (mut head, head_velocity) = projectile.calc_data(t0);
        let (tail, tail_velocity) = projectile.calc_data(t1);
        let tail_visual = projectile.calc_data(t2).0;

        if let Some((obstacle, obstacle_position, contact_position, _)) =
            find_obstacle(&(head, tail), projectile.shooter, &obstacles)
        {
            let contact_velocity =
                find_contact_velocity(contact_position, head, tail, head_velocity, tail_velocity);

            let angle =
                math::angle_difference(tail.angle_to(head), tail.angle_to(obstacle_position));

            audio.queue(AudioPlay {
                path: "sounds/hit_body".into(),
                volume: 1.2, // TODO: make it depend from momentum
                source: Some(obstacle_position),
                ..AudioPlay::DEFAULT
            });

            hits.add(
                obstacle,
                contact_velocity * projectile.config.fragment_mass(),
                angle,
            );

            head = contact_position;
            projectile.stopped = true;
        }

        update_transform(&projectile, head, tail_visual, &mut transform);

        if has_stopped(head_velocity) {
            projectile.stopped = true;
        }
    }
}

fn find_obstacle(
    projectaile: &(Vec2, Vec2),
    shooter: Option<Entity>,
    obstacles: &Query<(Entity, &Collision, &Transform), Without<Projectile>>,
) -> Option<(Entity, Vec2, Vec2, f32)> {
    let mut result: Option<(Entity, Vec2, Vec2, f32)> = None;

    for (entity, collision, transform) in obstacles.iter() {
        if shooter == Some(entity) {
            continue;
        }

        let obstacle = transform.translation.xy();
        let contact = obstacle.project_on(projectaile);

        if (obstacle - contact).is_shorter_than(collision.radius) {
            let tail_distance = obstacle.distance_squared(projectaile.1);

            if result.map_or(true, |o| o.3 > tail_distance) {
                result = Some((entity, obstacle, contact, tail_distance));
            }
        }
    }

    return result;
}

// TODO: test
fn find_contact_velocity(
    contact: Vec2,
    head: Vec2,
    tail: Vec2,
    head_velocity: Vec2,
    tail_velocity: Vec2,
) -> Vec2 {
    let d0 = contact.distance(tail);
    let d1 = contact.distance(head);
    return tail_velocity - d0 / (d0 + d1) * (tail_velocity - head_velocity);
}

fn update_transform(projectile: &Projectile, head: Vec2, tail: Vec2, transform: &mut Transform) {
    let center = (head + tail) / 2.0;
    transform.translation.x = center.x;
    transform.translation.y = center.y;
    transform.rotation = Quat::from_rotation_z(projectile.initial_velocity.angle());
    transform.scale.x = (head - tail).length();
    transform.scale.y = projectile.config.size;
}

fn has_stopped(velocity: Vec2) -> bool {
    return velocity.is_shorter_than(Projectile::VELOCITY_MIN);
}
