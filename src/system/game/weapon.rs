use crate::{
    command::{AudioPlay, ProjectileSpawn},
    component::{Actor, Weapon},
    model::{ActorActionsExt, TransformLite},
    util::ext::Vec2Ext,
};
use bevy::{
    ecs::system::{Query, Resource},
    math::Vec2,
    prelude::{Commands, Entity, Res, ResMut, Time, Transform},
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

const VELOCITY_DEVIATION_FACTOR: f32 = 0.1;
const DIRECTION_DEVIATION: f32 = 0.02;
const BARREL_LENGTH: f32 = 1.15; // TODO: don't hardcode

#[derive(Resource)]
pub struct WeaponData {
    rng: Pcg32,
}

impl Default for WeaponData {
    fn default() -> Self {
        return Self {
            rng: Pcg32::seed_from_u64(0),
        };
    }
}

pub fn weapon(
    mut query: Query<(Entity, &Actor, &Transform, &mut Weapon)>,
    mut commands: Commands,
    mut data: ResMut<WeaponData>,
    time: Res<Time>,
) {
    let now = time.elapsed();

    for (entity, actor, transform, mut weapon) in query.iter_mut() {
        if actor.actions.is_attacking() && weapon.fire(now) {
            let mut transform = TransformLite::from(transform);
            transform.translation += Vec2::from_length(BARREL_LENGTH, transform.direction);

            commands.add(AudioPlay {
                path: "sounds/shot.ogg",
                volume: 1.0,
                source: Some(transform.translation),
                ..AudioPlay::DEFAULT
            });

            for _ in 0..weapon.config.projectile.fragments {
                transform.direction = deviate_direction(&mut data.rng, transform.direction);

                commands.add(ProjectileSpawn {
                    config: weapon.config.projectile,
                    transform,
                    velocity: deviate_velocity(&mut data.rng, weapon.config.muzzle_velocity),
                    shooter: Some(entity),
                });
            }
        }
    }
}

fn deviate_velocity(rng: &mut Pcg32, velocity: f32) -> f32 {
    let min = 1.0 - VELOCITY_DEVIATION_FACTOR;
    let max = 1.0 + VELOCITY_DEVIATION_FACTOR;
    return velocity * rng.gen_range(min..max);
}

fn deviate_direction(rng: &mut Pcg32, direction: f32) -> f32 {
    let deviation = DIRECTION_DEVIATION;
    return direction + rng.gen_range(-deviation..deviation);
}
