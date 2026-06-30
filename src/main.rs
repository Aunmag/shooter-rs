mod command;
mod component;
mod data;
mod event;
mod map;
mod model;
mod plugin;
mod resource;
mod system;
mod util;

use crate::{
    command::CursorGrab,
    data::APP_TITLE,
    event::ActorDeathEvent,
    model::AppState,
    plugin::{
        bot::BotPlugin,
        camera_target::CameraTargetPlugin,
        collision::CollisionPlugin,
        debug::DebugPlugin,
        kinetics::KineticsPlugin,
        player::PlayerPlugin,
        scenario::{
            BenchZombiesScenario, Scenario, ScenarioPlugin, TestBotSpreadScenario, TestScenario,
            WavesScenario,
        },
        AudioTracker, AudioTrackerPlugin, BloodPlugin, BonusPlugin, BreathPlugin, CrosshairPlugin,
        DebugTweaksPlugin, ExplosionPlugin, FootstepsPlugin, HealthPlugin, HeartbeatPlugin,
        MainCamera, ParticlePlugin, ProjectilePlugin, SkipLoaderPlugin, StatusBarPlugin,
        TerrainPlugin, TileMapPlugin, UiNotificationPlugin, WeaponPlugin,
    },
    resource::{AssetStorage, AudioStorage, ScenarioSettings, Settings},
    util::ext::AppExt,
};
use bevy::{
    core_pipeline::core_2d::Camera2dBundle,
    ecs::world::{Command, World},
    log::LogPlugin,
    prelude::{App, AppExtStates, DefaultPlugins, IntoSystemConfigs, PluginGroup},
    render::texture::ImagePlugin,
    window::{Window, WindowPlugin, WindowResolution},
};

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
                        f32::from(settings.display.window_w),
                        f32::from(settings.display.window_h),
                    ),
                    present_mode: settings.display.present_mode(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
    );

    let scenario = match settings.game.scenario {
        ScenarioSettings::BenchZombies => Scenario::new(BenchZombiesScenario::default()),
        ScenarioSettings::Test => Scenario::new(TestScenario),
        ScenarioSettings::TestBotSpread => Scenario::new(TestBotSpreadScenario),
        ScenarioSettings::Waves => Scenario::new(WavesScenario::new(settings.game.level)),
    };

    if settings.game.debug {
        std::env::set_var("RUST_BACKTRACE", "1");
        application.add_plugins(DebugPlugin);
        application.add_plugins(DebugTweaksPlugin);
    }

    application
        .add_plugins(AudioTrackerPlugin)
        .add_plugins(BloodPlugin)
        .add_plugins(BonusPlugin)
        .add_plugins(BotPlugin)
        .add_plugins(BreathPlugin)
        .add_plugins(CameraTargetPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(CrosshairPlugin)
        .add_plugins(ExplosionPlugin)
        .add_plugins(FootstepsPlugin)
        .add_plugins(HealthPlugin)
        .add_plugins(HeartbeatPlugin)
        .add_plugins(KineticsPlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(ScenarioPlugin)
        .add_plugins(SkipLoaderPlugin)
        .add_plugins(StatusBarPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(TileMapPlugin)
        .add_plugins(UiNotificationPlugin)
        .add_plugins(WeaponPlugin)
        .add_event::<ActorDeathEvent>()
        .init_state::<AppState>()
        .insert_resource(AssetStorage::default())
        .insert_resource(AudioStorage::default())
        .insert_resource(AudioTracker::new(settings.audio.sources))
        .insert_resource(scenario)
        .insert_resource(settings)
        .add_state_system(AppState::Loading, system::loading::on_update())
        .add_state_system_enter(AppState::Game, init_game)
        .add_state_systems(AppState::Game, |s| {
            use crate::system::game::*;
            s.add(input);
            s.add(actor.after(crate::plugin::player::on_update));
            s.add(melee.after(crate::plugin::collision::on_update));
            s.add(ambience_fx());
        })
        .run();
}

fn init_log_plugin(settings: &Settings) -> LogPlugin {
    let mut log_plugin = LogPlugin::default();

    if settings.game.debug {
        if !log_plugin.filter.is_empty() {
            log_plugin.filter.push(',');
        }

        log_plugin.filter.push_str(env!("CARGO_PKG_NAME"));
        log_plugin.filter.push_str("=debug");
    }

    return log_plugin;
}

fn init_game(world: &mut World) {
    CursorGrab(true).apply(world);
    world.spawn(Camera2dBundle::default()).insert(MainCamera);
}
