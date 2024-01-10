use crate::{
    command::ProjectileSpawn,
    component::{Actor, Weapon},
    model::{ActorActionsExt, AudioPlay, TransformLite},
    resource::{AudioTracker, HitResource},
    util::ext::Vec2Ext,
};
use bevy::{
    ecs::system::{Deferred, Local, Query},
    math::{Vec2, Vec3Swizzles},
    prelude::{Commands, Entity, Res, Time, Transform},
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

const BARREL_LENGTH: f32 = 0.6; // TODO: don't hardcode

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

        if actor.actions.is_attacking() && weapon.try_fire(now) {
            let mut transform = TransformLite::from(transform);
            transform.translation += Vec2::from_length(BARREL_LENGTH, transform.direction);

            audio.queue(AudioPlay {
                path: "sounds/shot".into(),
                volume: 1.0,
                source: Some(transform.translation),
                ..AudioPlay::DEFAULT
            });

            for _ in 0..weapon.config.projectile.fragments {
                let deviation = weapon.config.generate_deviation(&mut data.rng);
                let velocity = weapon.config.generate_velocity(&mut data.rng);

                commands.add(ProjectileSpawn {
                    config: weapon.config.projectile,
                    transform: TransformLite::new(
                        transform.translation.x,
                        transform.translation.y,
                        transform.direction + deviation,
                    ),
                    velocity,
                    shooter: Some(entity),
                });
            }

            let mut recoil = weapon.get_recoil() * actor.config.recoil_factor / actor.skill;

            if data.rng.gen::<bool>() {
                recoil = -recoil;
            }

            hits.add(entity, Vec2::ZERO, recoil, true);
        }

        if !weapon.is_reloading() && (!weapon.has_ammo() || actor.actions.is_reloading()) {
            let reloading_duration = weapon
                .config
                .reloading_time
                .mul_f32(actor.config.reloading_speed)
                .div_f32(actor.skill);

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
