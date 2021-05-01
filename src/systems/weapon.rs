use crate::components::Actor;
use crate::components::ActorActions;
use crate::components::Weapon;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use crate::utils::Position;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::Entities;
use amethyst::ecs::Join;
use amethyst::ecs::Read;
use amethyst::ecs::ReadStorage;
use amethyst::ecs::System;
use amethyst::ecs::Write;
use amethyst::ecs::WriteStorage;
use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

const VELOCITY_DEVIATION_FACTOR: f32 = 0.1;
const DIRECTION_DEVIATION: f32 = 0.02;

pub struct WeaponSystem {
    randomizer: Pcg32,
}

impl WeaponSystem {
    pub fn new() -> Self {
        let randomizer_seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or_else(|e| e.duration().as_secs(), |t| t.as_secs());

        return Self {
            randomizer: Pcg32::seed_from_u64(randomizer_seed),
        };
    }

    fn deviate_velocity(&mut self, velocity: f32) -> f32 {
        let min = 1.0 - VELOCITY_DEVIATION_FACTOR;
        let max = 1.0 + VELOCITY_DEVIATION_FACTOR;
        return velocity * self.randomizer.gen_range(min..max) as f32;
    }

    fn deviate_direction(&mut self, direction: f32) -> f32 {
        let deviation = DIRECTION_DEVIATION;
        return direction + self.randomizer.gen_range(-deviation..deviation) as f32;
    }
}

impl<'a> System<'a> for WeaponSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, Actor>,
        ReadStorage<'a, Transform>,
        Write<'a, GameTaskResource>,
        WriteStorage<'a, Weapon>,
    );

    fn run(
        &mut self,
        (entities, time, actors, transforms, mut tasks, mut weapons): Self::SystemData,
    ) {
        let query = (&entities, &actors, &transforms, &mut weapons).join();

        for (entity, actor, transform, weapon) in query {
            if actor.actions.contains(ActorActions::ATTACK) && weapon.fire(time.absolute_time()) {
                let mut position = Position::from(transform);
                position.direction = self.deviate_direction(position.direction);

                tasks.push(GameTask::ProjectileSpawn {
                    position,
                    velocity: self.deviate_velocity(weapon.config.muzzle_velocity),
                    acceleration_factor: weapon.config.projectile.acceleration_factor,
                    shooter: Some(entity),
                });
            }
        }
    }
}
