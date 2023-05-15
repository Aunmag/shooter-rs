use crate::command::ProjectileSpawn;
use crate::component::Actor;
use crate::component::ActorAction;
use crate::component::Weapon;
use crate::model::TransformLite;
use bevy::ecs::system::Query;
use bevy::ecs::system::Resource;
use bevy::prelude::Commands;
use bevy::prelude::Entity;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::f32::consts::FRAC_PI_2;

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
        if actor.actions.contains(ActorAction::Attack) && weapon.fire(now) {
            let mut transform = TransformLite::from(transform);
            let (sin, cos) = (transform.direction + FRAC_PI_2).sin_cos();
            transform.translation.x += BARREL_LENGTH * cos;
            transform.translation.y += BARREL_LENGTH * sin;

            for _ in 0..8 {
                transform.direction = deviate_direction(&mut data.rng, transform.direction);

                commands.add(ProjectileSpawn {
                    transform,
                    velocity: deviate_velocity(&mut data.rng, weapon.config.muzzle_velocity),
                    acceleration_factor: weapon.config.projectile.acceleration_factor,
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
