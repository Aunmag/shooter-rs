mod command;
mod component;
mod config;

pub use self::{command::*, component::*, config::*};
use crate::{
    plugin::{
        collision::CollisionSystems, Actor, ActorActionsExt, AudioPlay, AudioTracker,
        ProjectilePhysics, ProjectileSpawn, ShellParticleSpawn,
    },
    resource::HitResource,
    state::AppState,
    util::ext::{AppExt, QuatExt, Vec2Ext},
};
use bevy::{
    ecs::system::{Deferred, Local, Query},
    math::{Vec2, Vec3Swizzles},
    prelude::{App, Commands, Entity, IntoScheduleConfigs, Plugin, Res, Time, Transform},
};
use rand::{RngExt, SeedableRng};
use rand_pcg::Pcg32;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update.after(CollisionSystems));
    }
}

struct Data {
    rng: Pcg32,
}

impl Default for Data {
    fn default() -> Self {
        return Self {
            rng: Pcg32::seed_from_u64(0),
        };
    }
}

fn on_update(
    mut data: Local<Data>,
    mut query: Query<(Entity, &Actor, &Transform, &mut Weapon)>,
    mut commands: Commands,
    mut hits: Deferred<HitResource>,
    audio: Res<AudioTracker>,
    time: Res<Time>,
) {
    crate::util::bench::bench!();
    let now = time.elapsed();

    for (entity, actor, transform, mut weapon) in query.iter_mut() {
        if weapon.is_reloading() && weapon.is_ready(now) {
            let was_armed = weapon.is_armed();
            weapon.complete_reloading(now);

            if !was_armed {
                audio.queue(AudioPlay {
                    path: "sounds/reloaded".into(),
                    volume: 0.8,
                    falloff: AudioPlay::FALLOFF_SHORTER,
                    source: Some(transform.translation.xy()),
                    ..AudioPlay::DEFAULT
                });
            }
        }

        if actor.actions.is_attacking() && weapon.try_fire(now) {
            let rotation = transform.rotation.angle_z();
            let offset = Vec2::from_length(Weapon::BARREL_LENGTH, rotation);
            let position = transform.translation.truncate() + offset;

            audio.queue(AudioPlay {
                path: format!("weapons/{}/shot", weapon.config.name).into(),
                volume: 1.0,
                falloff: AudioPlay::FALLOFF_LONGER,
                source: Some(position),
                ..AudioPlay::DEFAULT
            });

            if has_shells(&weapon) && weapon.config.has_bolt {
                commands.queue(ShellParticleSpawn(entity));
            }

            for _ in 0..weapon.config.projectile.fragments {
                let deviation = weapon.config.generate_deviation(&mut data.rng);
                let velocity = weapon.config.generate_velocity(&mut data.rng);

                commands.queue(ProjectileSpawn {
                    config: weapon.config.projectile,
                    position,
                    velocity: Vec2::from_angle(rotation + deviation) * velocity,
                    shooter: Some(entity),
                });
            }

            let recoil_push = transform.rotation.as_vec() * -weapon.get_recoil();
            let recoil_spin = if data.rng.random::<bool>() {
                actor.config.recoil_factor / actor.skill
            } else {
                actor.config.recoil_factor / -actor.skill
            };

            hits.add(entity, recoil_push, recoil_spin, true);
        }

        if !weapon.is_reloading() && (!weapon.has_ammo() || actor.actions.is_reloading()) {
            let reloading_duration = weapon.config.reloading_time.div_f32(actor.skill);
            weapon.reload(now, reloading_duration);
            audio.queue(AudioPlay {
                path: "sounds/reloading".into(),
                volume: 0.4,
                falloff: AudioPlay::FALLOFF_SHORTER,
                source: Some(transform.translation.xy()),
                duration: reloading_duration, // TODO: stop if weapon will be changed earlier
                ..AudioPlay::DEFAULT
            });

            if has_shells(&weapon) && !weapon.config.has_bolt {
                for _ in 0..weapon.config.ammo_capacity {
                    commands.queue(ShellParticleSpawn(entity));
                }
            }
        }
    }
}

fn has_shells(weapon: &Weapon) -> bool {
    return weapon.config.projectile.physics == ProjectilePhysics::Bullet;
}
