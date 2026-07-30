use crate::{
    plugin::{
        collision::Collision, projectile::state::ProjectileState, Actor, AudioPlay, AudioTracker,
        Explode, Projectile,
    },
    resource::HitResource,
    util::{ext::Vec2Ext, geometry::GeometryProjection, math::angle_factor_signed},
};
use bevy::{
    ecs::{
        entity::Entity,
        system::{Deferred, Query},
    },
    math::Vec3Swizzles,
    prelude::{Commands, Res, Time, Transform, Vec2, Without},
};
use std::time::Duration;

const TIME_DELTA_FOR_RENDER: Duration = Duration::from_millis(25); // 40 FPS
const SPIN_FACTOR: f32 = 1.5;

pub fn on_update(
    mut projectiles: Query<(Entity, &mut Projectile, &mut Transform)>,
    obstacles: Query<(Entity, &Collision, &Transform, &Actor), Without<Projectile>>,
    mut hits: Deferred<HitResource>,
    mut commands: Commands,
    audio: Res<AudioTracker>,
    time: Res<Time>,
) {
    crate::util::bench::bench!();
    // Abbreviations used:
    // j - projectile
    // s - projectile state
    // t - time (secs)
    // v - velocity (m/s)
    // d - traveled distance (m)
    // p - position
    //
    // _0 - at current frame
    // _1 - at previous frame
    // _2 - N frames ago - used only for visuals, not the actual physics

    let t0 = time.elapsed();
    let t1 = t0.saturating_sub(time.delta());
    let t2 = t0.saturating_sub(Duration::max(time.delta(), TIME_DELTA_FOR_RENDER));

    for (entity, mut j, mut transform) in projectiles.iter_mut() {
        if j.stopped {
            commands.entity(entity).despawn();
            continue;
        }

        let mut s0 = ProjectileState::calc(&j, t0);
        let mut p0 = s0.position();
        let mut stopped = s0.stopped();

        if !stopped {
            let p1 = ProjectileState::calc(&j, t1).position();

            if let Some(victim) = Victim::find(p0, p1, &obstacles, j.shooter) {
                stopped = true;

                // set projectile state at contact position. it will update velocity at that time too
                s0.update_by_traveled_distance(victim.contact - j.initial_position);
                p0 = s0.position(); // we updated state, so update position too

                // do not do double damage with explosives, since they dame by explosion not the hit
                if j.config.explosion.is_none() {
                    victim.hit(&s0, &audio, &mut hits);
                }
            }
        }

        let p2 = ProjectileState::calc(&j, t2).position();
        update_transform(&j, p0, p2, &mut transform);

        if stopped {
            j.stopped = true; // only stop (destroy) on next frame so player have time to see projectile hits the target

            if let Some(explosion) = &j.config.explosion {
                commands.queue(Explode {
                    config: explosion,
                    position: p0,
                    shooter: j.shooter,
                });
            }
        }
    }
}

struct Victim {
    entity: Entity,
    position: Vec2,
    contact: Vec2,
    distance_from_projectile_head: f32,
}

impl Victim {
    fn find(
        head: Vec2,
        tail: Vec2,
        obstacles: &Query<(Entity, &Collision, &Transform, &Actor), Without<Projectile>>,
        shooter: Option<Entity>,
    ) -> Option<Self> {
        let mut victim: Option<Victim> = None;

        let shooter_kind = shooter
            .and_then(|e| obstacles.get(e).ok())
            .map(|q| q.3.config.kind);

        for (entity, collision, transform, actor) in obstacles.iter() {
            if shooter == Some(entity) || shooter_kind == Some(actor.config.kind) {
                continue;
            }

            let obstacle = transform.translation.xy();

            let mut contact = obstacle.project_on_clamped(&(head, tail));
            let contact_distance_sq = obstacle.distance_squared(contact);
            let radius_sq = collision.radius * collision.radius;

            if contact_distance_sq < radius_sq {
                let head_distance = obstacle.distance_squared(head);

                if victim
                    .as_ref()
                    .is_none_or(|v| v.distance_from_projectile_head < head_distance)
                {
                    // contact position should be just at body radius, not inside it
                    contact -= Vec2::from_length(
                        (radius_sq - contact_distance_sq).sqrt(),
                        (head - tail).to_angle(),
                    );

                    victim = Some(Victim {
                        entity,
                        position: obstacle,
                        contact,
                        distance_from_projectile_head: head_distance,
                    });
                }
            }
        }

        return victim;
    }

    fn hit(&self, s: &ProjectileState, audio: &AudioTracker, hits: &mut HitResource) {
        let force = s.velocity() * s.projectile.config.fragment_mass();

        if force.is_zero() {
            return;
        }

        // hitting the side should spin more than hitting the center
        let spin = angle_factor_signed(s.projectile.initial_position, self.contact, self.position)
            * 2.0 // in our case it always returns a factor in [-0.5, 0.5] so we double it to get [-1, 1]
            * SPIN_FACTOR;

        hits.add(self.entity, force, -spin, false);

        audio.queue(AudioPlay {
            path: "sounds/hit_body".into(),
            volume: 1.2, // TODO: make it depend from momentum
            source: Some(self.position),
            ..AudioPlay::DEFAULT
        });
    }
}

fn update_transform(projectile: &Projectile, head: Vec2, tail: Vec2, transform: &mut Transform) {
    let center = (head + tail) / 2.0;
    transform.translation.x = center.x;
    transform.translation.y = center.y;
    transform.scale.x = (head - tail).length();
    transform.scale.y = projectile.config.size;
}
