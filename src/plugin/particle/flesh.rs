use crate::{
    component::Inertia,
    plugin::{
        particle::{Particle, ParticleConfig},
        BloodSpawn, TileBlend,
    },
    util::ext::{RngExt, Vec2Ext},
};
use bevy::{
    asset::AssetServer,
    ecs::{entity::Entity, system::Command, world::World},
    math::{Vec2, Vec3},
    prelude::{Time, Transform},
    sprite::{Sprite, SpriteBundle},
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
            commands.add(blood);
        }

        commands.add(TileBlend::Entity(entity));
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
        let Some(texture) = world.resource::<AssetServer>().get_handle(path) else {
            return;
        };

        let mut velocity = Vec2::from_length(
            rng.gen_range(VELOCITY_MIN..VELOCITY_MAX),
            rng.gen_range(0.0..TAU),
        );

        if let Some(inertia) = world.get::<Inertia>(self.0) {
            velocity += inertia.velocity / 2.0;
        }

        world
            .spawn(SpriteBundle {
                sprite: Sprite {
                    flip_x: rng.gen(),
                    flip_y: rng.gen(),
                    ..Default::default()
                },
                transform: Transform {
                    scale: Vec3::ZERO,
                    ..Default::default()
                },
                texture,
                ..Default::default()
            })
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
                until: now + rng.fuzz_duration(DURATION),
                scale: rng.fuzz(1.0),
            });
    }
}
