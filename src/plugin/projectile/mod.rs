mod command;
mod component;
mod config;
mod material;
mod state;
mod sys_update;
mod sys_whiz;

use self::material::ProjectileMaterial;
pub use self::{command::*, component::*, config::*};
use crate::{plugin::collision::CollisionSystems, util::ext::AppExt, AppState};
use bevy::{
    prelude::{App, IntoScheduleConfigs, Plugin},
    sprite_render::Material2dPlugin,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<ProjectileMaterial>::default());
        app.add_state_system(
            AppState::Game,
            sys_update::on_update.after(CollisionSystems),
        );
        app.add_state_system(AppState::Game, sys_whiz::on_update);
    }
}
