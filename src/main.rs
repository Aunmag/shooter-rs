#![warn(
    // basic
    clippy::all,
    clippy::cargo,
    // extra restrictions
    clippy::get_unwrap,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::string_to_string,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::verbose_file_reads,
)]
#![allow(
    clippy::cargo_common_metadata,
    clippy::multiple_crate_versions,
    clippy::needless_return,
    clippy::too_many_arguments,
    clippy::type_complexity
)]

mod command;
mod component;
mod data;
mod material;
mod model;
mod plugin;
mod resource;
mod scenario;
mod system;
mod util;

use crate::data::APP_TITLE;
use crate::material::HealthBarMaterial;
use crate::material::ProjectileMaterial;
use crate::model::AppState;
use crate::model::Arguments;
use crate::plugin::GameClientPlugin;
use crate::plugin::GameServerPlugin;
use crate::plugin::StressTestPlugin;
use crate::resource::AssetStorage;
use crate::resource::Config;
use crate::resource::GameType;
use crate::resource::NetResource;
use crate::util::ext::AppExt;
use bevy::prelude::App;
use bevy::prelude::DefaultPlugins;
use bevy::prelude::PluginGroup;
use bevy::render::texture::ImagePlugin;
use bevy::sprite::Material2dPlugin;
use bevy::window::Window;
use bevy::window::WindowPlugin;
use bevy::window::WindowResolution;
use clap::Parser;

fn main() {
    let arguments = Arguments::parse();

    log::debug!("Loading config from {}", arguments.config);
    let config = Config::load_from(&arguments.config).expect("Failed to load config");
    log::debug!("Config loaded: {:?}", config);

    let game_type = GameType::try_from(&arguments).expect("Wrong IPv4");
    log::debug!("Starting as {:?}", game_type);

    let net = match game_type {
        GameType::Server => {
            NetResource::new_as_server(&config.net).expect("Failed to start server")
        }
        GameType::Client => {
            NetResource::new_as_client(&config.net).expect("Failed to start client")
        }
    };

    let mut application = App::new();

    application
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: APP_TITLE.to_string(),
                        mode: config.display.mode(),
                        resolution: WindowResolution::new(
                            config.display.window_size_x,
                            config.display.window_size_y,
                        ),
                        present_mode: config.display.present_mode(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugin(Material2dPlugin::<HealthBarMaterial>::default())
        .add_plugin(Material2dPlugin::<ProjectileMaterial>::default())
        .add_state::<AppState>()
        .insert_resource(net)
        .insert_resource(system::game::CollisionSystemData::default())
        .insert_resource(system::game::WeaponData::default())
        .insert_resource(game_type)
        .insert_resource(AssetStorage::default())
        .insert_resource(config.clone())
        .add_state_system_enter(AppState::Loading, system::loading::on_enter)
        .add_state_system(AppState::Loading, system::loading::on_update)
        .add_state_system_enter(AppState::Game, system::game::on_enter);

    match game_type {
        GameType::Server => {
            log::info!("Starting as server");
            application.add_plugin(GameServerPlugin::new(config.net.server.sync_interval));

            if config.misc.with_stress_test {
                log::info!("Starting with StressTestPlugin plugin");
                application.add_plugin(StressTestPlugin);
            }
        }
        GameType::Client => {
            log::info!("Starting as client");
            application.add_plugin(GameClientPlugin);
        }
    };

    application.run();
}
