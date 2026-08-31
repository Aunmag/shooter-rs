use crate::{
    map::{ForestMap, Map, TestMap}, plugin::{ActorConfig, WeaponConfig, player::PlayerSpawn, scenario::ScenarioLogic},
};
use bevy::ecs::{system::Command, world::World};
use std::{any::Any, time::Duration};

pub struct TestScenario;

impl ScenarioLogic for TestScenario {
    fn on_enter(&mut self, _time: Duration, world: &mut World) -> Duration {
        ForestMap.generate(world);
        // TestMap.generate(world);

        PlayerSpawn {
            config: &ActorConfig::HUMAN,
            weapon: &WeaponConfig::AKS_74U,
            is_controllable: true,
        }
        .apply(world);

        return Duration::ZERO;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}
