mod command;
mod component;
mod config;
mod material;
mod sys_update;
mod sys_whiz;

use self::material::ProjectileMaterial;
pub use self::{command::*, component::*, config::*};
use crate::{model::AppState, system::game::collision_resolve, util::ext::AppExt};
use bevy::{
    prelude::{App, IntoSystemConfigs, Plugin},
    sprite::Material2dPlugin,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<ProjectileMaterial>::default());
        app.add_state_system(
            AppState::Game,
            sys_update::on_update.after(collision_resolve),
        );
        app.add_state_system(AppState::Game, sys_whiz::on_update);
    }
}
