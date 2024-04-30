use crate::{
    model::AudioPlay,
    plugin::{
        particle::{Particle, ParticleConfig},
        AudioTracker, TileBlend, Weapon,
    },
    util::ext::{RngExt, TransformExt, Vec2Ext},
};
use bevy::{
    asset::AssetServer,
    ecs::{
        entity::Entity,
        system::{Command, Commands},
        world::World,
    },
    math::{Vec2, Vec3},
    prelude::{Time, Transform},
    sprite::SpriteBundle,
};
use rand::Rng;
use std::{f32::consts::FRAC_PI_2, time::Duration};

const ROTATION: f32 = 0.3;
const VELOCITY: f32 = 0.5;
const VELOCITY_SPIN: f32 = 2.0;
const DURATION: Duration = Duration::from_millis(200);
const AUDIO_INTERVAL: Duration = Duration::from_millis(220);
const AUDIO_VOLUME: f32 = 0.3;

const PARTICLE_CONFIG: &ParticleConfig = &ParticleConfig {
    jump_factor: 0.5,
    on_destroy,
};

pub struct ShellParticleSpawn(pub Entity);

impl Command for ShellParticleSpawn {
    fn apply(self, world: &mut World) {
        let now = world.resource::<Time>().elapsed();
        let mut rng = rand::thread_rng();

        let Some((mut position, mut direction)) = world
            .get::<Transform>(self.0)
            .map(|t| (t.translation.truncate(), t.direction() - FRAC_PI_2))
        else {
            return;
        };

        let image = if world
            .get::<Weapon>(self.0)
            .map(|w| w.config.is_shotgun)
            .unwrap_or(false)
        {
            "particle/shell_shotgun.png"
        } else {
            "particle/shell.png"
        };

        let Some(texture) = world.resource::<AssetServer>().get_handle(image) else {
            return;
        };

        position += Vec2::from_length(Weapon::BARREL_LENGTH * 0.8, direction + FRAC_PI_2);
        direction += rng.gen_range_safely(-ROTATION, ROTATION);

        let velocity = Vec2::from_length(rng.fuzz(VELOCITY), direction);

        // TODO: fix
        // if let Some(kinetics) = world.get::<Inertia>(self.0) {
        //     velocity += kinetics.velocity / 2.0;
        // }

        world
            .spawn(SpriteBundle {
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
                rotation: direction,
                velocity,
                velocity_spin: Vec3::new(0.0, 0.0, rng.gen_range(-VELOCITY_SPIN..VELOCITY_SPIN)),
                since: now,
                until: now + rng.fuzz_duration(DURATION),
                scale: 1.0,
            });
    }
}

fn on_destroy(entity: Entity, point: Vec2, commands: &mut Commands) {
    commands.add(TileBlend::Entity(entity));

    commands.add(move |world: &mut World| {
        let mut time = world.resource::<Time>().elapsed();
        let mut rng = rand::thread_rng();
        let interval = rng.fuzz_duration(AUDIO_INTERVAL);
        let audio = world.resource::<AudioTracker>();
        let sound = AudioPlay {
            source: Some(point),
            volume: rng.fuzz(AUDIO_VOLUME),
            speed: rng.fuzz(1.0),
            ..AudioPlay::DEFAULT
        };

        audio.queue(AudioPlay {
            path: "sounds/shell_t0".into(),
            ..sound
        });

        if rng.gen() {
            time += interval;
            audio.queue_delayed(
                time,
                AudioPlay {
                    path: "sounds/shell_t1".into(),
                    ..sound
                },
            );
        }

        time += interval.mul_f32(0.6);
        audio.queue_delayed(
            time,
            AudioPlay {
                path: "sounds/shell_t2".into(),
                ..sound
            },
        );
    });
}
