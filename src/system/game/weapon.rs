use crate::{
    command::ProjectileSpawn,
    component::{Actor, Weapon},
    data::VIEW_DISTANCE,
    model::{ActorActionsExt, AudioPlay, TransformLite},
    resource::{AudioTracker, HitResource},
    util::{
        ext::{RngExt, TransformExt, Vec2Ext},
        GIZMOS,
    },
};
use bevy::{
    ecs::system::{Deferred, Local, Query},
    math::{Vec2, Vec3, Vec3Swizzles},
    prelude::{Commands, Entity, Res, Time, Transform},
    render::color::Color,
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

const BARREL_LENGTH: f32 = 0.6; // TODO: don't hardcode
const DEBUG_DEVIATION: bool = false;

pub struct WeaponSystemData {
    rng: Pcg32,
}

impl Default for WeaponSystemData {
    fn default() -> Self {
        return Self {
            rng: Pcg32::seed_from_u64(0),
        };
    }
}

pub fn weapon(
    mut data: Local<WeaponSystemData>,
    mut query: Query<(Entity, &Actor, &Transform, &mut Weapon)>,
    mut commands: Commands,
    mut hits: Deferred<HitResource>,
    audio: Res<AudioTracker>,
    time: Res<Time>,
) {
    let now = time.elapsed();

    for (entity, actor, transform, mut weapon) in query.iter_mut() {
        if weapon.is_reloading() && weapon.is_ready(now) {
            let was_armed = weapon.is_armed();
            weapon.complete_reloading(now);

            if !was_armed {
                audio.queue(AudioPlay {
                    path: "sounds/reloaded".into(),
                    volume: 0.8,
                    source: Some(transform.translation.xy()),
                    ..AudioPlay::DEFAULT
                });
            }
        }

        // get deviation before shoot, because it will increase after
        let deviation = weapon.get_deviation(now);

        if DEBUG_DEVIATION {
            debug_deviation(transform, deviation);
        }

        if actor.actions.is_attacking() && weapon.try_fire(now) {
            let mut position = TransformLite::from(transform);
            position.translation += Vec2::from_length(BARREL_LENGTH, position.direction);

            audio.queue(AudioPlay {
                path: "sounds/shot".into(),
                volume: 1.0,
                source: Some(position.translation),
                ..AudioPlay::DEFAULT
            });

            for _ in 0..weapon.config.projectile.fragments {
                let deviation = data.rng.gen_range_safely(-deviation, deviation);
                let velocity = weapon.config.generate_velocity(&mut data.rng);

                commands.add(ProjectileSpawn {
                    config: weapon.config.projectile,
                    transform: TransformLite::new(
                        position.translation.x,
                        position.translation.y,
                        position.direction + deviation,
                    ),
                    velocity,
                    shooter: Some(entity),
                });
            }

            let recoil_push = transform.rotation * Vec3::new(-weapon.get_recoil(), 0.0, 0.0);
            let recoil_spin = if data.rng.gen::<bool>() {
                actor.config.recoil_factor / actor.skill
            } else {
                actor.config.recoil_factor / -actor.skill
            };

            hits.add(entity, recoil_push.truncate(), recoil_spin, true);
        }

        if !weapon.is_reloading() && (!weapon.has_ammo() || actor.actions.is_reloading()) {
            let reloading_duration = weapon.config.reloading_time.div_f32(actor.skill);
            weapon.reload(now, reloading_duration);
            audio.queue(AudioPlay {
                path: "sounds/reloading".into(),
                volume: 0.4,
                source: Some(transform.translation.xy()),
                duration: reloading_duration, // TODO: stop if weapon will be changed earlier
            });

            continue;
        }
    }
}

fn debug_deviation(transform: &Transform, deviation: f32) {
    let p = transform.translation.truncate();
    let d = transform.direction();
    let l = VIEW_DISTANCE / 2.0;
    let color = Color::WHITE.with_a(0.5);
    GIZMOS.ln(p, p + Vec2::from_length(l, d + deviation), color);
    GIZMOS.ln(p, p + Vec2::from_length(l, d - deviation), color);
}
