mod data;
mod map;
mod plugin;
mod resource;
mod state;
mod util;

use crate::{
    data::APP_TITLE,
    plugin::{
        bot::BotPlugin,
        camera_target::CameraTargetPlugin,
        collision::CollisionPlugin,
        debug::DebugPlugin,
        kinetics::KineticsPlugin,
        player::PlayerPlugin,
        scenario::{
            BenchProjectilesScenario, BenchZombiesScenario, Scenario, ScenarioPlugin,
            TestBotSpreadScenario, TestScenario, WavesScenario,
        },
        ActorPlugin, AmbienceFxPlugin, AudioPlugin, BloodPlugin, BonusPlugin, BreathPlugin,
        CrosshairPlugin, CursorGrab, DebugTweaksPlugin, ExplosionPlugin, FootstepsPlugin,
        HealthPlugin, HeartbeatPlugin, InputPlugin, LoadingPlugin, MainCamera, MeleePlugin,
        ParticlePlugin, ProjectilePlugin, SkipLoaderPlugin, StatusBarPlugin, TerrainPlugin,
        TileMapPlugin, UiNotificationPlugin, WeaponPlugin,
    },
    resource::{AssetStorage, ScenarioSettings, Settings},
    state::AppState,
    util::ext::AppExt,
};
use bevy::{
    core_pipeline::core_2d::Camera2d,
    ecs::{system::Command, world::World},
    log::LogPlugin,
    prelude::{App, AppExtStates, DefaultPlugins, PluginGroup},
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
        ScenarioSettings::BenchProjectiles => Scenario::new(BenchProjectilesScenario::default()),
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
        .add_plugins(ActorPlugin)
        .add_plugins(AmbienceFxPlugin)
        .add_plugins(AudioPlugin::new(settings.audio.sources))
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
        .add_plugins(InputPlugin)
        .add_plugins(KineticsPlugin)
        .add_plugins(LoadingPlugin)
        .add_plugins(MeleePlugin)
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
        .init_state::<AppState>()
        .insert_resource(AssetStorage::default())
        .insert_resource(scenario)
        .insert_resource(settings)
        .add_state_system_enter(AppState::Game, init_game)
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
    world.spawn(Camera2d).insert(MainCamera);
}
