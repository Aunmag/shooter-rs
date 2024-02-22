mod command;
mod component;
mod data;
mod event;
mod model;
mod plugin;
mod resource;
mod scenario;
mod system;
mod util;

use crate::{
    component::ProjectileMaterial,
    data::APP_TITLE,
    event::ActorDeathEvent,
    model::AppState,
    plugin::{
        BloodPlugin, BreathPlugin, CameraTargetPlugin, CrosshairPlugin, DebugPlugin,
        FootstepsPlugin, HeartbeatPlugin, LaserSightPlugin, StatusBarPlugin, UiNotificationPlugin,
    },
    resource::{AssetStorage, AudioStorage, AudioTracker, GameMode, Scenario, Settings},
    scenario::{BenchScenario, EmptyScenario, WavesScenario},
    util::ext::AppExt,
};
use bevy::{
    gizmos::GizmoConfig,
    log::LogPlugin,
    prelude::{App, DefaultPlugins, IntoSystem, IntoSystemConfigs, PluginGroup, Update},
    render::texture::ImagePlugin,
    sprite::Material2dPlugin,
    window::{Window, WindowPlugin, WindowResolution},
};

fn main() {
    // TODO: init logger earlier
    log::info!("Loading settings...");

    let settings = match Settings::load() {
        Ok(settings) => {
            log::info!("Settings loaded: {:?}", settings);
            settings
        }
        Err(error) => {
            log::error!("{:?}", error);
            log::warn!("Default settings will be used");
            Settings::default()
        }
    };

    let mut application = App::new();

    application.add_plugins(
        DefaultPlugins
            .set(init_log_plugin(&settings))
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: APP_TITLE.to_string(),
                    mode: settings.display.mode(),
                    resolution: WindowResolution::new(
                        f32::from(settings.display.window_size_x),
                        f32::from(settings.display.window_size_y),
                    ),
                    present_mode: settings.display.present_mode(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
    );

    let mut scenario = None;

    for mode in &settings.game.modes {
        log::info!("Starting with game mode: {:?}", mode);

        match &mode {
            GameMode::Debug => {
                std::env::set_var("RUST_BACKTRACE", "1");
                application.add_plugins(DebugPlugin);
            }
            GameMode::Bench => {
                scenario = Some(Scenario::new(BenchScenario::default()));
            }
            GameMode::Waves => {
                scenario = Some(Scenario::new(WavesScenario::new()));
            }
            GameMode::LaserSight => {}
        }
    }

    application.insert_resource(scenario.unwrap_or_else(|| Scenario::new(EmptyScenario)));

    application
        .add_plugins(BloodPlugin)
        .add_plugins(BreathPlugin)
        .add_plugins(CameraTargetPlugin)
        .add_plugins(CrosshairPlugin)
        .add_plugins(FootstepsPlugin)
        .add_plugins(HeartbeatPlugin)
        .add_plugins(LaserSightPlugin)
        .add_plugins(StatusBarPlugin)
        .add_plugins(UiNotificationPlugin)
        .add_plugins(Material2dPlugin::<ProjectileMaterial>::default())
        .add_state::<AppState>()
        .add_event::<ActorDeathEvent>()
        .insert_resource(AssetStorage::default())
        .insert_resource(AudioStorage::default())
        .insert_resource(AudioTracker::new(settings.audio.sources))
        .insert_resource(settings)
        .insert_resource(GizmoConfig {
            line_width: 3.0,
            ..Default::default()
        })
        .add_systems(Update, system::sys::audio)
        .add_state_system(AppState::Loading, system::loading::on_update())
        .add_state_system_enter(AppState::Game, system::game::on_enter)
        .add_state_systems(AppState::Game, |s| {
            use crate::system::{bot, game::*};
            s.add(input);
            s.add(health);
            s.add(player);
            s.add(actor.after(player));
            s.add(inertia.after(actor));
            s.add(collision_find.pipe(collision_resolve).after(inertia));
            s.add(weapon.after(collision_resolve));
            s.add(melee.after(collision_resolve));
            s.add(projectile.after(collision_resolve));
            s.add(projectile_whiz);
            s.add(bonus_image);
            s.add(bonus_label);
            s.add(bonus.after(collision_resolve));
            s.add(ambience_fx());
            s.add(terrain);
            s.add(scenario);
            s.add(bot::analyze);
            s.add(bot::operate);
            s.add(bot::voice);
        })
        .run();
}

fn init_log_plugin(settings: &Settings) -> LogPlugin {
    let mut log_plugin = LogPlugin::default();

    if settings.game.modes.contains(&GameMode::Debug) {
        if !log_plugin.filter.is_empty() {
            log_plugin.filter.push(',');
        }

        log_plugin.filter.push_str(env!("CARGO_PKG_NAME"));
        log_plugin.filter.push_str("=debug");
    }

    return log_plugin;
}
