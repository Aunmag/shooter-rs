use crate::{
    command::{ActorBotSet, ActorSet, BonusSpawn},
    component::{ActorConfig, Player},
    data::FONT_PATH,
    model::{AppState, TransformLite},
    resource::AudioTracker,
    util::{
        ext::{AppExt, Vec2Ext},
        Timer, GIZMOS,
    },
};
use bevy::{
    app::{App, Plugin},
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    ecs::{schedule::SystemConfigs, system::Local},
    gizmos::gizmos::Gizmos,
    input::Input,
    prelude::{
        AssetServer, Color, Commands, Component, IntoSystemConfigs, KeyCode, Query, Res, Startup,
        TextBundle, Update, Vec2, With,
    },
    text::{Text, TextSection, TextStyle},
    time::Time,
    transform::components::Transform,
};
use std::time::Duration;

const INTERVAL: Duration = Duration::from_millis(500);

#[derive(Component)]
struct FpsText;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, application: &mut App) {
        application
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(EntityCountDiagnosticsPlugin)
            .add_systems(Startup, startup)
            .add_systems(Update, update_diagnostics())
            .add_systems(Update, render_gizmos_static)
            .add_state_system(AppState::Game, update_input);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = TextStyle {
        font: asset_server.get_handle(FONT_PATH).unwrap_or_default(),
        font_size: 30.0,
        color: Color::WHITE,
    };

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("FPS: ", style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::new("\nEntities: ", style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::new("\nAudio sources: ", style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::new(
                "\n\
                \nSpawn weapon: [G]\
                \nSpawn human : [H] group: [+SHIFT]\
                \nSpawn zombie: [J] group: [+SHIFT]\
                ",
                style,
            ),
        ]),
        FpsText,
    ));
}

fn update_diagnostics_inner(
    diagnostics: Res<DiagnosticsStore>,
    audio_tracker: Res<AudioTracker>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|v| v.average())
        .unwrap_or(-1.0);

    let entities = diagnostics
        .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|v| v.value())
        .unwrap_or(-1.0);

    for mut text in &mut query {
        text.sections[1].value = format!("{:.0}", fps);
        text.sections[3].value = format!("{:.0}", entities);
        text.sections[5].value = format!("{}", audio_tracker.playing);
    }
}

fn update_diagnostics() -> SystemConfigs {
    return update_diagnostics_inner.run_if(|mut r: Local<Timer>, t: Res<Time>| {
        return r.next_if_ready(t.elapsed(), || INTERVAL);
    });
}

fn render_gizmos_static(mut gizmos: Gizmos) {
    GIZMOS.render(&mut gizmos);
}

fn update_input(
    players: Query<&Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    let spawn = if keyboard.just_pressed(KeyCode::G) {
        Spawn::Bonus
    } else if keyboard.just_pressed(KeyCode::H) {
        Spawn::Human
    } else if keyboard.just_pressed(KeyCode::J) {
        Spawn::Zombie
    } else {
        return;
    };

    let group = if keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight].into_iter()) {
        10
    } else {
        1
    };

    let mut position = players
        .iter()
        .next()
        .map(TransformLite::from)
        .unwrap_or_default();

    position.translation += Vec2::from_length(3.0, position.direction);

    match spawn {
        Spawn::Bonus => {
            commands.add(BonusSpawn::new(position.translation, 6)); // TODO: don't hardcode level
        }
        Spawn::Human => {
            spawn_actors(&mut commands, position, &ActorConfig::HUMAN, group);
        }
        Spawn::Zombie => {
            spawn_actors(&mut commands, position, &ActorConfig::ZOMBIE, group);
        }
    }
}

fn spawn_actors(
    commands: &mut Commands,
    transform: TransformLite,
    config: &'static ActorConfig,
    group: u8,
) {
    for _ in 0..group {
        let entity = commands.spawn_empty().id();

        commands.add(ActorSet {
            entity,
            config,
            skill: 1.0,
            transform,
        });

        commands.add(ActorBotSet(entity));
    }
}

enum Spawn {
    Bonus,
    Human,
    Zombie,
}
