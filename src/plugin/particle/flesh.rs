use crate::{
    plugin::{
        kinetics::Kinetics,
        particle::{Particle, ParticleConfig},
        BloodSpawn, TileBlend,
    },
    util::ext::{Fuzz, Vec2Ext},
};
use bevy::{
    asset::AssetServer,
    ecs::{entity::Entity, system::Command, world::World},
    math::{Vec2, Vec3},
    prelude::{Time, Transform},
    sprite::Sprite,
};
use rand::Rng;
use std::{f32::consts::TAU, time::Duration};

const VELOCITY_MIN: f32 = 1.0;
const VELOCITY_MAX: f32 = 3.0;
const VELOCITY_SPIN: f32 = 2.5;
const DURATION: Duration = Duration::from_millis(400);

const PARTICLE_CONFIG: &ParticleConfig = &ParticleConfig {
    jump_factor: 1.5,
    on_destroy: |entity, point, commands| {
        if let Some(blood) = BloodSpawn::new(point, 0.2) {
            commands.queue(blood);
        }

        commands.queue(TileBlend::Entity(entity));
    },
};

pub struct FleshParticleSpawn(pub Entity);

impl Command for FleshParticleSpawn {
    fn apply(self, world: &mut World) {
        let now = world.resource::<Time>().elapsed();
        let mut rng = rand::thread_rng();

        let Some(position) = world
            .get::<Transform>(self.0)
            .map(|t| t.translation.truncate())
        else {
            return;
        };

        // TODO: find available automatically
        let path = format!("particle/flesh_{}.png", rng.gen_range(0..=5));
        let Some(image) = world.resource::<AssetServer>().get_handle(path) else {
            return;
        };

        let mut velocity = Vec2::from_length(
            rng.gen_range(VELOCITY_MIN..VELOCITY_MAX),
            rng.gen_range(0.0..TAU),
        );

        if let Some(kinetics) = world.get::<Kinetics>(self.0) {
            velocity += kinetics.velocity / 2.0;
        }

        world
            .spawn((
                Sprite {
                    image,
                    flip_x: rng.gen(),
                    flip_y: rng.gen(),
                    ..Default::default()
                },
                Transform {
                    scale: Vec3::ZERO,
                    ..Default::default()
                },
            ))
            .insert(Particle {
                config: PARTICLE_CONFIG,
                position,
                rotation: rng.gen_range(0.0..TAU),
                velocity,
                velocity_spin: Vec3::new(
                    rng.gen_range(-VELOCITY_SPIN..VELOCITY_SPIN) / 2.0,
                    rng.gen_range(-VELOCITY_SPIN..VELOCITY_SPIN) / 2.0,
                    rng.gen_range(-VELOCITY_SPIN..VELOCITY_SPIN),
                ),
                since: now,
                until: now + DURATION.fuzz(&mut rng),
                scale: 1.0.fuzz(&mut rng),
            });
    }
}
