mod command;
mod component;
mod data;
mod material;
mod model;
mod plugin;
mod resource;
mod system;
mod util;

use crate::data::APP_TITLE;
use crate::material::ProjectileMaterial;
use crate::model::AppState;
use crate::model::Arguments;
use crate::plugin::StressTestPlugin;
use crate::resource::Config;
use crate::resource::EntityConverter;
use crate::resource::GameType;
use crate::resource::LoadingAssets;
use crate::resource::NetResource;
use crate::resource::PositionUpdateResource;
use crate::util::ext::AppExt;
use bevy::prelude::App;
use bevy::prelude::DefaultPlugins;
use bevy::prelude::IntoChainSystem;
use bevy::prelude::ParallelSystemDescriptorCoercion;
use bevy::prelude::SystemSet;
use bevy::prelude::WindowDescriptor;
use bevy::render::texture::ImageSettings;
use bevy::sprite::Material2dPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use clap::Parser;
use rand::SeedableRng;
use rand_pcg::Pcg32;

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

    App::new()
        .insert_resource(WindowDescriptor {
            title: APP_TITLE.to_string(),
            mode: config.display.mode(),
            width: config.display.window_size_x,
            height: config.display.window_size_y,
            present_mode: config.display.present_mode(),
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<ProjectileMaterial>::default())
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin_if(config.misc.with_stress_test, || StressTestPlugin)
        .insert_resource(net)
        .insert_resource(Pcg32::seed_from_u64(0))
        .insert_resource(system::game::CollisionSystemData::default())
        .insert_resource(system::net::PositionUpdateSendData::new(config.net.server.sync_interval)) // TODO: on server only
        .insert_resource(system::net::InputSendData::default()) // TODO: on client only
        .insert_resource(game_type)
        .insert_resource(LoadingAssets::default())
        .insert_resource(EntityConverter::default()) // TODO: on client only
        .insert_resource(PositionUpdateResource::default()) // TODO: on client only
        .insert_resource(config)
        .add_state(AppState::Loading)
        // loading
        .add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .with_system(system::loading::on_enter)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
                .with_system(system::loading::on_update)
        )
        // game
        .add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(system::game::on_enter)
        )
        .add_system_set(
            SystemSet::on_resume(AppState::Game)
                .with_system(system::game::on_resume)
        )
        .add_system_set(init_game_systems(&game_type))
        .run();
}

fn init_game_systems(game_type: &GameType) -> SystemSet {
    return match game_type {
        GameType::Server => init_server_game_systems(),
        GameType::Client => init_client_game_systems(),
    };
}

fn init_server_game_systems() -> SystemSet {
    use system::game::*;
    use system::net::*;

    let collision = collision_find.chain(collision_resolve).label("collision");

    return SystemSet::on_update(AppState::Game)
        .with_system(input)
        .with_system(health)
        .with_system(ai)
        .with_system(player.after(input))
        .with_system(actor.after(player))
        .with_system(inertia.after(actor))
        .with_system(collision.after(inertia))
        .with_system(weapon.after("collision"))
        .with_system(projectile.chain(projectile_hit).after("collision"))
        .with_system(position_update_send.after("collision"))
        .with_system(message_receive)
        .with_system(connection_update)
        .with_system(camera.after("collision"))
        .with_system(terrain);
}

fn init_client_game_systems() -> SystemSet {
    use system::game::*;
    use system::net::*;

    return SystemSet::on_update(AppState::Game)
        .with_system(input)
        .with_system(interpolation)
        .with_system(player.after(input))
        .with_system(actor.after(player).after(interpolation))
        .with_system(inertia.after(actor))
        .with_system(input_send.after(player).after(actor))
        .with_system(projectile.chain(projectile_hit).after(inertia))
        .with_system(message_receive)
        .with_system(
            position_update_receive
                .after(message_receive)
                .after(inertia),
        )
        .with_system(connection_update)
        .with_system(camera.after(inertia))
        .with_system(terrain);
}
