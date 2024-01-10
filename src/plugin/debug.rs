use crate::{
    command::{ActorBotSet, ActorSet, BonusSpawn, WeaponSet},
    component::{ActorConfig, ActorKind, WeaponConfig},
    material::CrosshairMaterial,
    model::{AppState, TransformLite},
    resource::AudioTracker,
    util::{ext::AppExt, Timer, GIZMOS},
};
use bevy::{
    app::{App, Plugin},
    asset::Handle,
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    ecs::{schedule::SystemConfigs, system::Local, world::World},
    gizmos::gizmos::Gizmos,
    input::Input,
    prelude::{
        Commands, Component, IntoSystemConfigs, KeyCode, Query, Res, Startup, TextBundle, Update,
        With,
    },
    text::{Text, TextSection, TextStyle},
    time::Time,
    transform::components::Transform,
};
use rand::Rng;
use std::time::Duration;

const INTERVAL: Duration = Duration::from_millis(500);

const ZOMBIE_PISTOL_CHANCE: f32 = 0.1;
const ZOMBIE_RIFLE_CHANCE: f32 = 0.02;
const HUMAN_RIFLE_CHANCE: f32 = 0.1;

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

fn startup(world: &mut World) {
    let style = TextStyle {
        font_size: 30.0,
        ..Default::default()
    };

    world.spawn((
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
    crosshairs: Query<&Transform, With<Handle<CrosshairMaterial>>>,
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

    let position = crosshairs
        .iter()
        .next()
        .map(TransformLite::from)
        .unwrap_or_default();

    match spawn {
        Spawn::Bonus => {
            commands.add(BonusSpawn::new(position.translation, u8::MAX));
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

        commands.add(ActorBotSet { entity, skill: 1.0 });

        let weapon_chance = rand::thread_rng().gen::<f32>();

        let weapon = match config.kind {
            ActorKind::Human => {
                if weapon_chance < HUMAN_RIFLE_CHANCE {
                    Some(&WeaponConfig::AKS_74U)
                } else {
                    Some(&WeaponConfig::PM)
                }
            }
            ActorKind::Zombie => {
                if weapon_chance < ZOMBIE_RIFLE_CHANCE {
                    Some(&WeaponConfig::AKS_74U)
                } else if weapon_chance < ZOMBIE_PISTOL_CHANCE {
                    Some(&WeaponConfig::PM)
                } else {
                    None
                }
            }
        };

        if let Some(weapon) = weapon {
            commands.add(WeaponSet {
                entity,
                weapon: Some(weapon),
            });
        }
    }
}

enum Spawn {
    Bonus,
    Human,
    Zombie,
}
