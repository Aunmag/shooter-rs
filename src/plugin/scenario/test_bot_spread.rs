use crate::{
    command::ActorSet,
    component::ActorConfig,
    map::{Map, TestMap},
    plugin::{bot::ActorBotSet, camera_target::CameraTarget, scenario::ScenarioLogic},
};
use bevy::{
    ecs::world::{Command, World},
    math::Vec2,
    transform::components::Transform,
};
use std::{any::Any, time::Duration};

pub struct TestBotSpreadScenario;

impl ScenarioLogic for TestBotSpreadScenario {
    fn on_enter(&mut self, world: &mut World) -> Duration {
        TestMap.generate(world);

        world
            .spawn_empty()
            .insert(Transform::default())
            .insert(CameraTarget::default());

        for _ in 0..128 {
            let entity = world.spawn_empty().id();

            ActorSet {
                entity,
                config: &ActorConfig::ZOMBIE,
                position: Vec2::ZERO,
                rotation: 0.0,
            }
            .apply(world);

            ActorBotSet { entity }.apply(world); // TODO: disable idle
        }

        return Duration::ZERO;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}
