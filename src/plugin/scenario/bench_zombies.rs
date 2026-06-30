use crate::{
    command::ActorSet,
    component::ActorConfig,
    map::{ForestMap, Map},
    plugin::{
        bot::ActorBotSet,
        player::PlayerSpawn,
        scenario::{bench_utils::Bench, ScenarioLogic},
        WeaponConfig,
    },
};
use bevy::{
    ecs::world::{Command, World},
    math::Vec2,
    prelude::Commands,
};
use std::{any::Any, time::Duration};

const SPAWN_BATCH: usize = 100;
const SPAWN_MAX: usize = 3000;

#[derive(Default)]
pub struct BenchZombiesScenario {
    inner: Bench,
}

impl BenchZombiesScenario {
    fn spawn_batch(&mut self, commands: &mut Commands) {
        for _ in 0..SPAWN_BATCH {
            let entity = commands.spawn_empty().id();

            commands.add(ActorSet {
                entity,
                config: &ActorConfig::ZOMBIE,
                position: Vec2::ZERO,
                rotation: 0.0,
            });

            commands.add(ActorBotSet { entity });

            self.inner.spawned += 1;
        }
    }
}

impl ScenarioLogic for BenchZombiesScenario {
    fn on_enter(&mut self, _time: Duration, world: &mut World) -> Duration {
        ForestMap.generate(world);

        // TODO: just spawn spectator
        PlayerSpawn {
            config: &ActorConfig::HUMAN,
            weapon: &WeaponConfig::AKS_74U,
            is_controllable: false,
        }
        .apply(world);

        return Duration::ZERO;
    }

    fn on_constant_update(&mut self, time: Duration, commands: &mut Commands) {
        if self.inner.try_next(time) {
            self.spawn_batch(commands);
        }

        if self.inner.spawned >= SPAWN_MAX {
            self.inner.finish(commands);
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}
