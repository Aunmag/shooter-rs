use crate::{
    component::ActorConfig,
    map::{Map, TestMap},
    plugin::{player::PlayerSpawn, scenario::ScenarioLogic, WeaponConfig},
};
use bevy::ecs::world::{Command, World};
use std::{any::Any, time::Duration};

pub struct TestScenario;

impl ScenarioLogic for TestScenario {
    fn on_enter(&mut self, world: &mut World) -> Duration {
        TestMap.generate(world);

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
