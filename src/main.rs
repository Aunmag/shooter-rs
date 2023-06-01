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
    clippy::type_complexity
)]

mod command;
mod component;
mod data;
mod event;
mod material;
mod model;
mod plugin;
mod resource;
mod scenario;
mod system;
mod util;

use crate::{
    data::{APP_TITLE, CONFIG_PATH},
    event::ActorDeathEvent,
    material::{ProjectileMaterial, StatusBarMaterial},
    model::AppState,
    plugin::DebugPlugin,
    resource::{
        AssetStorage, AudioStorage, AudioTracker, Config, HeartbeatResource, Misc, Scenario,
    },
    scenario::WavesScenario,
    util::ext::AppExt,
};
use bevy::{
    prelude::{App, DefaultPlugins, IntoPipeSystem, IntoSystemConfig, PluginGroup},
    render::texture::ImagePlugin,
    sprite::Material2dPlugin,
    window::{Window, WindowPlugin, WindowResolution},
};

fn main() {
    log::debug!("Loading config from {}", CONFIG_PATH);
    let config = Config::load_from(CONFIG_PATH).expect("Failed to load config");
    log::debug!("Config loaded: {:?}", config);

    let mut app = App::new();

    if config.misc.debug {
        log::info!("Starting with debug mode");
        std::env::set_var("RUST_BACKTRACE", "1");
        app.add_plugin(DebugPlugin);
    }

    app.add_plugins(
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
    .add_plugin(Material2dPlugin::<StatusBarMaterial>::default())
    .add_plugin(Material2dPlugin::<ProjectileMaterial>::default())
    .add_state::<AppState>()
    .add_event::<ActorDeathEvent>()
    .insert_resource(AssetStorage::default())
    .insert_resource(AudioStorage::default())
    .insert_resource(AudioTracker::default())
    .insert_resource(HeartbeatResource::default())
    .insert_resource(Misc::default())
    .insert_resource(config)
    .insert_resource(Scenario::new(WavesScenario::new()))
    .insert_resource(system::bot::TargetFindData::default())
    .insert_resource(system::bot::TargetUpdateData::default())
    .insert_resource(system::game::AmbienceFxData::default())
    .insert_resource(system::game::CollisionSystemData::default())
    .insert_resource(system::game::WeaponData::default())
    .add_system(system::sys::audio_tracker)
    .add_system(system::ui::notification)
    .add_state_system_enter(AppState::Loading, system::loading::on_enter)
    .add_state_system(AppState::Loading, system::loading::on_update)
    .add_state_system_enter(AppState::Game, system::game::on_enter)
    .add_state_systems(AppState::Game, |s| {
        use crate::system::{bot, game::*};
        s.add(input);
        s.add(health);
        s.add(player.after(input));
        s.add(actor.after(player));
        s.add(inertia.after(actor));
        s.add(collision_find.pipe(collision_resolve).after(inertia));
        s.add(weapon.after(collision_resolve));
        s.add(melee.after(collision_resolve));
        s.add(projectile.pipe(projectile_hit).after(collision_resolve));
        s.add(bonus_image);
        s.add(bonus_label);
        s.add(bonus.after(collision_resolve));
        s.add(camera.after(collision_resolve));
        s.add(status_bar);
        s.add(breath);
        s.add(footsteps);
        s.add(heartbeat);
        s.add(ambience_fx);
        s.add(terrain);
        s.add(scenario);
        s.add(bot::target_find);
        s.add(bot::target_update.after(bot::target_find));
        s.add(bot::target_follow.after(bot::target_update));
        s.add(bot::sound);
    })
    .run();
}
