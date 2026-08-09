use crate::{
    plugin::{
        particle::{Particle, ParticleConfig},
        SpriteDissolve,
    },
    resource::AssetStorage,
    util::ext::{Fuzz, Vec2Ext},
};
use bevy::{
    color::Srgba,
    ecs::{system::Command, world::World},
    math::{Vec2, Vec3},
    prelude::{Time, Transform},
    sprite::Sprite,
};
use rand::RngExt;
use std::{f32::consts::TAU, time::Duration};

const VELOCITY_SPIN: f32 = 2.5;
const COLOR: Srgba = Srgba::new(0.3, 0.22, 0.13, 1.0);

const PARTICLE_CONFIG: &ParticleConfig = &ParticleConfig {
    jump_factor: 1.5,
    on_destroy: |e, _, c| {
        c.entity(e).insert(SpriteDissolve);
    },
};

pub struct DirtParticleSpawn {
    pub amount: u8,
    pub position: Vec2,
    pub velocity_min: f32,
    pub velocity_max: f32,
    pub duration: Duration,
    pub size_max: f32,
}

impl Command for DirtParticleSpawn {
    type Out = ();

    fn apply(self, world: &mut World) {
        let now = world.resource::<Time>().elapsed();
        let mut rng = rand::rng();
        let image = world.resource::<AssetStorage>().dummy_image().clone();

        for _ in 0..self.amount {
            world
                .spawn((
                    Sprite {
                        image: image.clone(),
                        color: COLOR.fuzz(&mut rng).into(),
                        ..Default::default()
                    },
                    Transform {
                        scale: Vec3::ZERO,
                        ..Default::default()
                    },
                ))
                .insert(Particle {
                    config: PARTICLE_CONFIG,
                    position: self.position,
                    rotation: rng.random_range(0.0..TAU),
                    velocity: Vec2::from_length(
                        rng.random_range(self.velocity_min..self.velocity_max),
                        rng.random_range(0.0..TAU),
                    ),
                    velocity_spin: Vec3::new(
                        rng.random_range(-VELOCITY_SPIN..VELOCITY_SPIN) / 2.0,
                        rng.random_range(-VELOCITY_SPIN..VELOCITY_SPIN) / 2.0,
                        rng.random_range(-VELOCITY_SPIN..VELOCITY_SPIN),
                    ),
                    since: now,
                    until: now + self.duration.fuzz(&mut rng),
                    scale: rng.random_range(1.0..f32::max(self.size_max, 1.0)),
                });
        }
    }
}
