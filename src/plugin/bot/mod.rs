mod command;
mod component;
mod config;
mod sys_analyze;
mod sys_detour;
mod sys_operate;
mod voice;

pub use self::{command::*, component::Bot, config::*};
use crate::{plugin::bot::voice::BotVoicePlugin, util::ext::AppExt, AppState};
use bevy::app::{App, Plugin};

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BotVoicePlugin);
        app.add_state_system(AppState::Game, sys_analyze::on_update);
        app.add_state_system(AppState::Game, sys_operate::on_update);
        app.add_state_system(AppState::Game, sys_detour::on_update());
    }
}
