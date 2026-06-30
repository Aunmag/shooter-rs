use crate::{
    command::ActorSet,
    component::{Actor, ActorConfig},
    model::TransformLite,
    plugin::{
        camera_target::CameraTarget,
        scenario::{bench_utils::Bench, ScenarioLogic},
        ProjectileSpawn, WeaponConfig,
    },
    util::{ext::Vec2Ext, Timer},
};
use bevy::{
    ecs::{
        query::With,
        world::{Command, World},
    },
    math::Vec2,
    prelude::Commands,
    transform::components::Transform,
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::{any::Any, f32::consts::TAU, time::Duration};

const SEED: u64 = 4487;
const SHOT_INTERVAL: Duration = Duration::from_millis((60.0 / WEAPON.fire_rate * 1000.0) as u64);
const SPAWN_BATCH: usize = 50;
const SPAWN_MAX: usize = 1500;
const WORLD_SIZE: f32 = 100.0;
const WEAPON: &WeaponConfig = &WeaponConfig::AK_74M;

pub struct BenchProjectilesScenario {
    inner: Bench,
    rng: Pcg32,
    next_shot: Timer,
}

impl Default for BenchProjectilesScenario {
    fn default() -> Self {
        return Self {
            inner: Bench::default(),
            rng: Pcg32::seed_from_u64(SEED),
            next_shot: Timer::default(),
        };
    }
}

impl BenchProjectilesScenario {
    fn spawn_shooter(&mut self, commands: &mut Commands) {
        let entity = commands.spawn_empty().id();
        let s = WORLD_SIZE / 2.0;
        let x = self.rng.gen_range(-s..s);
        let y = self.rng.gen_range(-s..s);
        let r = self.rng.gen_range(0.0..TAU);

        commands.add(ActorSet {
            entity,
            config: &ActorConfig::ZOMBIE,
            position: Vec2::new(x, y),
            rotation: r,
        });

        self.inner.spawned += 1;
    }
}

impl ScenarioLogic for BenchProjectilesScenario {
    fn on_enter(&mut self, _time: Duration, world: &mut World) -> Duration {
        world
            .spawn_empty()
            .insert(Transform::default())
            .insert(CameraTarget::default());

        for _ in 0..5 {
            self.spawn_shooter(&mut world.commands());
        }

        return Duration::ZERO;
    }

    fn on_constant_update(&mut self, time: Duration, commands: &mut Commands) {
        if self.inner.try_next(time) {
            for _ in 0..SPAWN_BATCH {
                self.spawn_shooter(commands);
            }
        }

        if self.next_shot.try_next_add(time, SHOT_INTERVAL) {
            commands.add(|world: &mut World| {
                let transforms = world
                    .query_filtered::<&Transform, With<Actor>>()
                    .iter(world)
                    .map(|t| {
                        let mut t = TransformLite::from(t);
                        t.position += Vec2::from_length(0.5, t.rotation);
                        return t;
                    })
                    .collect::<Vec<_>>();

                for transform in transforms {
                    ProjectileSpawn {
                        config: WEAPON.projectile,
                        transform,
                        velocity: WEAPON.muzzle_velocity,
                        shooter: None,
                    }
                    .apply(world);
                }
            });
        }

        if self.inner.spawned > SPAWN_MAX {
            self.inner.finish(commands);
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}
