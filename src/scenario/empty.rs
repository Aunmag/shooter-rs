use crate::{
    command::{ActorPlayerSet, ActorSet},
    component::ActorConfig,
    model::TransformLite,
    resource::ScenarioLogic,
};
use bevy::ecs::system::Commands;
use std::{any::Any, time::Duration};

pub struct EmptyScenario;

impl ScenarioLogic for EmptyScenario {
    fn on_start(&mut self, commands: &mut Commands) -> Duration {
        let entity = commands.spawn_empty().id();

        commands.add(ActorSet {
            entity,
            config: &ActorConfig::HUMAN,
            skill: 1.0,
            transform: TransformLite::default(),
        });

        commands.add(ActorPlayerSet {
            entity,
            is_controllable: true,
        });

        return Duration::ZERO;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}
