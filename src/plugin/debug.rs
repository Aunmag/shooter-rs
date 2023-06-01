use crate::{
    command::{ActorBotSet, ActorSet, BonusSpawn},
    component::{ActorConfig, Player},
    data::FONT_PATH,
    model::{AppState, TransformLite},
    util::ext::{AppExt, Vec2Ext},
};
use bevy::{
    app::{App, Plugin},
    diagnostic::{
        Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    input::Input,
    prelude::{
        AssetServer, Color, Commands, Component, KeyCode, Query, Res, TextBundle, Vec2, With,
    },
    text::{Text, TextSection, TextStyle},
    transform::components::Transform,
};

#[derive(Component)]
struct FpsText;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(EntityCountDiagnosticsPlugin::default())
            .add_plugin(SystemInformationDiagnosticsPlugin::default())
            .add_startup_system(startup)
            .add_system(update_diagnostics)
            .add_state_system(AppState::Game, update_input);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = TextStyle {
        font: asset_server.get_handle(FONT_PATH),
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("\nCPU: ", style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::new("\nMEM: ", style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::new("\nFPS: ", style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::new("\nEntities: ", style.clone()),
            TextSection::from_style(style),
        ]),
        FpsText,
    ));
}

fn update_diagnostics(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    let cpu = diagnostics
        .get(SystemInformationDiagnosticsPlugin::CPU_USAGE)
        .and_then(|v| v.average())
        .unwrap_or(-1.0);

    let mem = diagnostics
        .get(SystemInformationDiagnosticsPlugin::MEM_USAGE)
        .and_then(|v| v.average())
        .unwrap_or(-1.0);

    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|v| v.average())
        .unwrap_or(-1.0);

    let entities = diagnostics
        .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|v| v.value())
        .unwrap_or(-1.0);

    for mut text in &mut query {
        text.sections[1].value = format!("{:.2}", cpu);
        text.sections[3].value = format!("{:.2}", mem);
        text.sections[5].value = format!("{:.2}", fps);
        text.sections[7].value = format!("{:.2}", entities);
    }
}

fn update_input(
    players: Query<&Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::Key0) {
        let player_position = players
            .iter()
            .next()
            .map(TransformLite::from)
            .unwrap_or_default();

        spawn_bonus(player_position, &mut commands);
    }

    if keyboard.just_pressed(KeyCode::Key1) {
        spawn_actors(10, &mut commands);
    }

    if keyboard.just_pressed(KeyCode::Key2) {
        spawn_actors(100, &mut commands);
    }

    if keyboard.just_pressed(KeyCode::Key3) {
        spawn_actors(1000, &mut commands);
    }
}

fn spawn_bonus(mut position: TransformLite, commands: &mut Commands) {
    position.translation += Vec2::from_length(2.0, position.direction);
    commands.add(BonusSpawn::new(position.translation, u8::MAX));
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
