use crate::{
    command::{ActorBotSet, ActorSet},
    component::ActorConfig,
    model::{AppState, TransformLite},
    util::ext::AppExt,
};
use bevy::{
    app::{App, Plugin},
    diagnostic::{Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    input::Input,
    prelude::{Commands, KeyCode, Res},
};

pub struct StressTestPlugin;

impl Plugin for StressTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(EntityCountDiagnosticsPlugin::default())
            .add_state_system(AppState::Game, system);
    }
}

fn system(keyboard: Res<Input<KeyCode>>, diagnostics: Res<Diagnostics>, mut commands: Commands) {
    if keyboard.just_pressed(KeyCode::Key1) {
        spawn_actors(10, &mut commands);
    }

    if keyboard.just_pressed(KeyCode::Key2) {
        spawn_actors(100, &mut commands);
    }

    if keyboard.just_pressed(KeyCode::Key3) {
        spawn_actors(1000, &mut commands);
    }

    if keyboard.just_pressed(KeyCode::Equals) {
        log(&diagnostics);
    }
}

fn spawn_actors(count: usize, commands: &mut Commands) {
    for _ in 0..count {
        let entity = commands.spawn_empty().id();

        commands.add(ActorSet {
            entity,
            config: ActorConfig::ZOMBIE,
            skill: 1.0,
            transform: TransformLite::default(),
        });

        commands.add(ActorBotSet(entity));
    }

    log::info!("Spawned +{} entities", count)
}

fn log(diagnostics: &Diagnostics) {
    let entities = diagnostics
        .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|d| d.value())
        .unwrap_or(-1.0)
        .floor();

    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|f| f.average())
        .unwrap_or(-1.0)
        .floor();

    log::info!("Entities: {} | FPS: {}", entities, fps);
}
