use crate::command::ProjectileSpawn;
use crate::component::Actor;
use crate::component::ActorActions;
use crate::component::Weapon;
use crate::model::Position;
use bevy::ecs::system::Query;
use bevy::prelude::Commands;
use bevy::prelude::Entity;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use rand::Rng;
use rand_pcg::Pcg32;
use std::f32::consts::FRAC_PI_2;

const VELOCITY_DEVIATION_FACTOR: f32 = 0.1;
const DIRECTION_DEVIATION: f32 = 0.02;
const BARREL_LENGTH: f32 = 1.15; // TODO: don't hardcode

pub fn weapon(
    mut query: Query<(Entity, &Actor, &Transform, &mut Weapon)>,
    mut commands: Commands,
    mut randomizer: ResMut<Pcg32>,
    time: Res<Time>,
) {
    let now = time.time_since_startup();

    for (entity, actor, transform, mut weapon) in query.iter_mut() {
        if actor.actions.contains(ActorActions::ATTACK) && weapon.fire(now) {
            let mut position = Position::from(transform);
            let (sin, cos) = (position.direction + FRAC_PI_2).sin_cos();
            position.x += BARREL_LENGTH * cos;
            position.y += BARREL_LENGTH * sin;

            for _ in 0..8 {
                position.direction = deviate_direction(&mut randomizer, position.direction);

                commands.add(ProjectileSpawn {
                    position,
                    velocity: deviate_velocity(&mut randomizer, weapon.config.muzzle_velocity),
                    acceleration_factor: weapon.config.projectile.acceleration_factor,
                    shooter: Some(entity),
                });
            }
        }
    }
}

fn deviate_velocity(randomizer: &mut Pcg32, velocity: f32) -> f32 {
    let min = 1.0 - VELOCITY_DEVIATION_FACTOR;
    let max = 1.0 + VELOCITY_DEVIATION_FACTOR;
    return velocity * randomizer.gen_range(min..max) as f32;
}

fn deviate_direction(randomizer: &mut Pcg32, direction: f32) -> f32 {
    let deviation = DIRECTION_DEVIATION;
    return direction + randomizer.gen_range(-deviation..deviation) as f32;
}
