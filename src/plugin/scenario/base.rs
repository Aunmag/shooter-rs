use crate::{
    command::ActorSet,
    component::ActorConfig,
    model::{AppState, TransformLite},
    plugin::{player::PlayerSet, WeaponConfig, WeaponSet},
    util::ext::AppExt,
};
use bevy::prelude::{App, Commands, Plugin};

pub struct BaseScenarioPlugin;

impl Plugin for BaseScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system_enter(AppState::Game, on_init);
    }
}

fn on_init(mut commands: Commands) {
    let entity = commands.spawn_empty().id();

    commands.add(ActorSet {
        entity,
        config: &ActorConfig::HUMAN,
        transform: TransformLite::default(),
    });

    commands.add(PlayerSet {
        entity,
        is_controllable: true,
    });

    commands.add(WeaponSet {
        entity,
        weapon: Some(&WeaponConfig::PM),
    });
}
