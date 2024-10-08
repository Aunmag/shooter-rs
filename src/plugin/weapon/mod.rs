mod command;
mod component;
mod config;

pub use self::{command::*, component::*, config::*};
use crate::{
    component::Actor,
    model::{ActorActionsExt, AppState, AudioPlay, TransformLite},
    plugin::{AudioTracker, ProjectileSpawn, ShellParticleSpawn},
    resource::HitResource,
    util::ext::{AppExt, QuatExt, Vec2Ext},
};
use bevy::{
    ecs::system::{Deferred, Local, Query},
    math::{Vec2, Vec3Swizzles},
    prelude::{App, Commands, Entity, IntoSystemConfigs, Plugin, Res, Time, Transform},
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(
            AppState::Game,
            on_update.after(crate::plugin::collision::on_update),
        );
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
            let rotation = transform.rotation.angle_z();
            let offset = Vec2::from_length(Weapon::BARREL_LENGTH, rotation);
            let position = transform.translation.truncate() + offset;

            audio.queue(AudioPlay {
                path: "sounds/shot".into(),
                volume: 1.0,
                source: Some(position),
                ..AudioPlay::DEFAULT
            });

            if weapon.config.has_bolt {
                commands.add(ShellParticleSpawn(entity));
            }

            for _ in 0..weapon.config.projectile.fragments {
                let deviation = weapon.config.generate_deviation(&mut data.rng);
                let velocity = weapon.config.generate_velocity(&mut data.rng);

                commands.add(ProjectileSpawn {
                    config: weapon.config.projectile,
                    transform: TransformLite {
                        position,
                        rotation: rotation + deviation,
                    },
                    velocity,
                    shooter: Some(entity),
                });
            }

            let recoil = weapon.get_recoil();
            let recoil_push = Vec2::new(-recoil, 0.0).rotate_by_quat(transform.rotation);
            let recoil_spin = if data.rng.gen::<bool>() {
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
                source: Some(transform.translation.xy()),
                duration: reloading_duration, // TODO: stop if weapon will be changed earlier
                ..AudioPlay::DEFAULT
            });

            if !weapon.config.has_bolt {
                for _ in 0..weapon.config.ammo_capacity {
                    commands.add(ShellParticleSpawn(entity));
                }
            }

            continue;
        }
    }
}
