use crate::{model::AppState, util::ext::AppExt};
use bevy::{
    app::Update,
    prelude::{App, Commands, IntoSystemConfigs, Plugin, Res, Time, World},
};

pub struct WavesScenarioPlugin;

impl Plugin for WavesScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system_enter(AppState::Game, on_init);
    }
}

fn on_init(mut commands: Commands) {
    // TODO: fix
}
