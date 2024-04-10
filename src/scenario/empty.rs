use crate::{
    command::{ActorPlayerSet, ActorSet},
    component::ActorConfig,
    model::TransformLite,
    plugin::{WeaponConfig, WeaponSet},
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
            transform: TransformLite::default(),
        });

        commands.add(ActorPlayerSet {
            entity,
            is_controllable: true,
        });

        commands.add(WeaponSet {
            entity,
            weapon: Some(&WeaponConfig::PM),
        });

        return Duration::ZERO;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}
