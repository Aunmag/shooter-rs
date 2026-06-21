use crate::{
    command::ActorSet,
    component::ActorConfig,
    plugin::{bot::ActorBotSet, camera_target::CameraTarget},
    resource::ScenarioLogic,
};
use bevy::{ecs::system::Commands, math::Vec2, transform::components::Transform};
use std::{any::Any, time::Duration};

pub struct TestBotSpreadScenario;

impl ScenarioLogic for TestBotSpreadScenario {
    fn on_start(&mut self, commands: &mut Commands) -> Duration {
        commands
            .spawn_empty()
            .insert(Transform::default())
            .insert(CameraTarget::default());

        for _ in 0..128 {
            let entity = commands.spawn_empty().id();

            commands.add(ActorSet {
                entity,
                config: &ActorConfig::ZOMBIE,
                position: Vec2::ZERO,
                rotation: 0.0,
            });

            commands.add(ActorBotSet { entity }); // TODO: disable idle
        }

        return Duration::ZERO;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}
