use crate::{
    command::ActorSet,
    component::{ActorConfig, ActorKind},
    model::{AppState, TransformLite},
    plugin::{
        bot::ActorBotSet, AudioTracker, BonusSpawn, Crosshair, TileMap, WeaponConfig, WeaponSet,
    },
    util::{ext::AppExt, Timer},
};
use bevy::{
    app::{App, Plugin},
    asset::Handle,
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    ecs::{
        schedule::SystemConfigs,
        system::{Local, ResMut, Resource},
        world::World,
    },
    gizmos::gizmos::Gizmos,
    input::Input,
    prelude::{
        Color, Commands, Component, IntoSystemConfigs, KeyCode, Query, Res, Startup, TextBundle,
        Update, Vec2, With,
    },
    text::{Text, TextSection, TextStyle},
    time::Time,
    transform::components::Transform,
};
use rand::Rng;
use std::{
    sync::{Mutex, OnceLock},
    time::Duration,
};

const UPDATE_TEXT_INTERVAL: Duration = Duration::from_millis(500);

const ZOMBIE_PISTOL_CHANCE: f32 = 0.1;
const ZOMBIE_RIFLE_CHANCE: f32 = 0.02;
const HUMAN_RIFLE_CHANCE: f32 = 0.1;

static DRAW_QUEUE: OnceLock<Mutex<Vec<Shape>>> = OnceLock::new();

#[derive(Component)]
struct DiagnosticsText;

#[derive(Default, Resource)]
struct DiagnosticsData {
    fps: Option<i32>,
    entities: Option<i32>,
    audio_sources: Option<i32>,
    map_layers: Option<i32>,
    map_tiles: Option<i32>,
    map_queue: Option<i32>,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, application: &mut App) {
        application
            .insert_resource(DiagnosticsData::default())
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(EntityCountDiagnosticsPlugin)
            .add_systems(Startup, startup)
            .add_systems(Update, update_diagnostics_data)
            .add_systems(Update, update_diagnostics_text())
            .add_systems(Update, render_debug_shapes)
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
            TextSection::new("\n\nMap. Layers: ", style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::new("\nMap. Tiles: ", style.clone()),
            TextSection::from_style(style.clone()),
            TextSection::new("\nMap. Queue: ", style.clone()),
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
        DiagnosticsText,
    ));
}

fn update_diagnostics_data(
    diagnostics: Res<DiagnosticsStore>,
    audio_tracker: Res<AudioTracker>,
    tile_map: Res<TileMap>,
    mut data: ResMut<DiagnosticsData>,
) {
    if let Some(value) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.value())
    {
        let value = value as i32;
        data.fps = Some(i32::min(value, data.fps.unwrap_or(value)));
    }

    if let Some(value) = diagnostics
        .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|d| d.value())
    {
        let value = value as i32;
        data.entities = Some(i32::max(value, data.entities.unwrap_or(value)));
    }

    {
        let value = audio_tracker.playing as i32;
        data.audio_sources = Some(i32::max(value, data.audio_sources.unwrap_or(value)));
    }

    {
        let value = tile_map.count_layers() as i32;
        data.map_layers = Some(i32::max(value, data.map_layers.unwrap_or(value)));
    }

    {
        let value = tile_map.count_tiles() as i32;
        data.map_tiles = Some(i32::max(value, data.map_tiles.unwrap_or(value)));
    }

    {
        let value = tile_map.count_queue() as i32;
        data.map_queue = Some(i32::max(value, data.map_queue.unwrap_or(value)));
    }
}

fn update_diagnostics_text_inner(
    mut data: ResMut<DiagnosticsData>,
    mut query: Query<&mut Text, With<DiagnosticsText>>,
) {
    for mut text in &mut query {
        text.sections[1].value = format!("{}", data.fps.unwrap_or(-1));
        text.sections[3].value = format!("{}", data.entities.unwrap_or(-1));
        text.sections[5].value = format!("{}", data.audio_sources.unwrap_or(-1));
        text.sections[7].value = format!("{}", data.map_layers.unwrap_or(-1));
        text.sections[9].value = format!("{}", data.map_tiles.unwrap_or(-1));
        text.sections[11].value = format!("{}", data.map_queue.unwrap_or(-1));
    }

    data.fps = None;
    data.entities = None;
    data.audio_sources = None;
    data.map_layers = None;
    data.map_tiles = None;
    data.map_queue = None;
}

fn update_diagnostics_text() -> SystemConfigs {
    return update_diagnostics_text_inner
        .after(update_diagnostics_data)
        .run_if(|mut r: Local<Timer>, t: Res<Time>| {
            return r.next_if_ready(t.elapsed(), || UPDATE_TEXT_INTERVAL);
        });
}

fn render_debug_shapes(mut gizmos: Gizmos) {
    let Ok(mut queue) = get_draw_queue().lock() else {
        return;
    };

    for shape in queue.drain(..) {
        match shape {
            Shape::Line(head, tail, color) => {
                gizmos.line_2d(head, tail, color);
            }
            Shape::Circle(center, radius, color) => {
                gizmos.circle_2d(center, radius, color).segments(24);
            }
        }
    }
}

fn update_input(
    crosshairs: Query<&Transform, With<Handle<Crosshair>>>,
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
            transform,
        });

        commands.add(ActorBotSet { entity });

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

fn get_draw_queue() -> &'static Mutex<Vec<Shape>> {
    return DRAW_QUEUE.get_or_init(|| Mutex::new(Vec::new()));
}

pub fn debug_line(head: Vec2, tail: Vec2, color: Color) {
    if let Ok(queue) = get_draw_queue().lock().as_mut() {
        queue.push(Shape::Line(head, tail, color));
    }
}

pub fn debug_circle(center: Vec2, radius: f32, color: Color) {
    if let Ok(queue) = get_draw_queue().lock().as_mut() {
        queue.push(Shape::Circle(center, radius, color));
    }
}

enum Shape {
    Line(Vec2, Vec2, Color),
    Circle(Vec2, f32, Color),
}
