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
    data::APP_TITLE,
    event::ActorDeathEvent,
    model::AppState,
    plugin::{
        bot::BotPlugin, camera_target::CameraTargetPlugin, collision::CollisionPlugin,
        debug::DebugPlugin, kinetics::KineticsPlugin, player::PlayerPlugin,
        scenario::WavesScenarioPlugin, AudioTracker, AudioTrackerPlugin, BloodPlugin, BonusPlugin,
        BreathPlugin, CrosshairPlugin, FootstepsPlugin, HealthPlugin, HeartbeatPlugin,
        ParticlePlugin, ProjectilePlugin, StatusBarPlugin, TerrainPlugin, TileMapPlugin,
        UiNotificationPlugin, WeaponPlugin,
    },
    resource::{AssetStorage, AudioStorage, GameMode, Scenario, Settings},
    scenario::{BenchScenario, WavesScenario},
    util::ext::AppExt,
};
use bevy::{
    gizmos::GizmoConfig,
    log::LogPlugin,
    prelude::{App, DefaultPlugins, IntoSystemConfigs, PluginGroup},
    render::texture::ImagePlugin,
    window::{Window, WindowPlugin, WindowResolution},
};
use plugin::{command_scheduler::CommandSchedulerPlugin, scenario::BaseScenarioPlugin};

fn main() {
    // TODO: init logger earlier
    let settings = Settings::load_or_default();
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

    // TODO: do later
    for mode in &settings.game.modes {
        log::info!("Starting with game mode: {:?}", mode);

        match &mode {
            GameMode::Debug => {
                std::env::set_var("RUST_BACKTRACE", "1");
                application.add_plugins(DebugPlugin);
            }
            GameMode::Bench => {
                application.insert_resource(Scenario::new(BenchScenario::default()));
            }
            GameMode::Waves => {
                application.add_plugins(WavesScenarioPlugin);
                application.insert_resource(Scenario::new(WavesScenario::new()));
            }
        }
    }

    application
        .add_plugins(AudioTrackerPlugin)
        .add_plugins(BaseScenarioPlugin)
        .add_plugins(BloodPlugin)
        .add_plugins(BonusPlugin)
        .add_plugins(BotPlugin)
        .add_plugins(BreathPlugin)
        .add_plugins(CameraTargetPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(CommandSchedulerPlugin)
        .add_plugins(CrosshairPlugin)
        .add_plugins(FootstepsPlugin)
        .add_plugins(HealthPlugin)
        .add_plugins(HeartbeatPlugin)
        .add_plugins(KineticsPlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(StatusBarPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(TileMapPlugin)
        .add_plugins(UiNotificationPlugin)
        .add_plugins(WeaponPlugin)
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
        .add_state_system(AppState::Loading, system::loading::on_update())
        .add_state_system_enter(AppState::Game, system::game::on_enter)
        .add_state_systems(AppState::Game, |s| {
            use crate::system::game::*;
            s.add(input);
            s.add(actor.after(crate::plugin::player::on_update));
            s.add(melee.after(crate::plugin::collision::on_update));
            s.add(ambience_fx());
            s.add(scenario);
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
