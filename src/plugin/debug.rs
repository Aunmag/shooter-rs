use crate::{
    plugin::{
        bot::ActorBotSet, ActorConfig, ActorKind, ActorSet, AudioTracker, BonusSpawn, Crosshair,
        Explode, ProjectileConfig, TileMap, WeaponConfig, WeaponSet,
    },
    state::AppState,
    util::{ext::AppExt, Timer, Transform2D},
};
use bevy::{
    app::{App, Plugin},
    color::Srgba,
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    ecs::{
        entity::Entity,
        resource::Resource,
        schedule::ScheduleConfigs,
        system::{Local, ResMut, ScheduleSystem},
        world::World,
    },
    gizmos::gizmos::Gizmos,
    input::ButtonInput,
    prelude::{
        Commands, Component, DefaultGizmoConfigGroup, GizmoConfigStore, IntoScheduleConfigs,
        KeyCode, Query, Res, Update, Vec2, With,
    },
    sprite::MeshMaterial2d,
    text::TextSpan,
    time::Time,
    transform::components::Transform,
    ui::widget::{Text, TextUiWriter},
};
use rand::seq::SliceRandom;
use std::{
    sync::{Mutex, OnceLock},
    time::Duration,
};

const UPDATE_TEXT_INTERVAL: Duration = Duration::from_millis(500);

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
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(EntityCountDiagnosticsPlugin)
            .add_state_system_enter(AppState::Game, on_init)
            .add_systems(Update, update_diagnostics_data)
            .add_systems(Update, update_diagnostics_text())
            .add_systems(Update, render_debug_shapes)
            .add_state_system(AppState::Game, update_input);
    }
}

fn on_init(world: &mut World) {
    world
        .resource_mut::<GizmoConfigStore>()
        .config_mut::<DefaultGizmoConfigGroup>()
        .0
        .line
        .width = 3.0;

    world
        .spawn((DiagnosticsText, Text::new("")))
        .with_child(TextSpan::new("FPS: "))
        .with_child(TextSpan::new("?"))
        .with_child(TextSpan::new("\nEntities: "))
        .with_child(TextSpan::new("?"))
        .with_child(TextSpan::new("\nAudio sources: "))
        .with_child(TextSpan::new("?"))
        .with_child(TextSpan::new("\n\nMap. Layers: "))
        .with_child(TextSpan::new("?"))
        .with_child(TextSpan::new("\nMap. Tiles: "))
        .with_child(TextSpan::new("?"))
        .with_child(TextSpan::new("\nMap. Queue: "))
        .with_child(TextSpan::new("?"))
        .with_child(TextSpan::new(
            "\n\
            \nSpawn weapon: [G]\
            \nSpawn human : [H] group: [+SHIFT]\
            \nSpawn zombie: [J] group: [+SHIFT]\
            \nExplode: [T]\
            ",
        ));
}

fn update_diagnostics_data(
    diagnostics: Res<DiagnosticsStore>,
    audio_tracker: Res<AudioTracker>,
    tile_map: Res<TileMap>,
    mut data: ResMut<DiagnosticsData>,
) {
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.value())
    {
        let value = value as i32;
        data.fps = Some(i32::min(value, data.fps.unwrap_or(value)));
    }

    if let Some(value) = diagnostics
        .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
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
    mut query: Query<Entity, With<DiagnosticsText>>,
    mut text_writer: TextUiWriter,
) {
    for entity in &mut query {
        *text_writer.text(entity, 2) = data.fps.unwrap_or(-1).to_string();
        *text_writer.text(entity, 4) = data.entities.unwrap_or(-1).to_string();
        *text_writer.text(entity, 6) = data.audio_sources.unwrap_or(-1).to_string();
        *text_writer.text(entity, 8) = data.map_layers.unwrap_or(-1).to_string();
        *text_writer.text(entity, 10) = data.map_tiles.unwrap_or(-1).to_string();
        *text_writer.text(entity, 12) = data.map_queue.unwrap_or(-1).to_string();
    }

    data.fps = None;
    data.entities = None;
    data.audio_sources = None;
    data.map_layers = None;
    data.map_tiles = None;
    data.map_queue = None;
}

fn update_diagnostics_text() -> ScheduleConfigs<ScheduleSystem> {
    return update_diagnostics_text_inner
        .after(update_diagnostics_data)
        .run_if(|mut r: Local<Timer>, t: Res<Time>| {
            return r.try_next_set(t.elapsed(), || UPDATE_TEXT_INTERVAL);
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
                gizmos.circle_2d(center, radius, color).resolution(24);
            }
        }
    }
}

fn update_input(
    crosshairs: Query<&Transform, With<MeshMaterial2d<Crosshair>>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    let spawn = if keyboard.just_pressed(KeyCode::KeyG) {
        Spawn::Bonus
    } else if keyboard.just_pressed(KeyCode::KeyH) {
        Spawn::Human
    } else if keyboard.just_pressed(KeyCode::KeyJ) {
        Spawn::Zombie
    } else if keyboard.just_pressed(KeyCode::KeyT) {
        Spawn::Explosion
    } else {
        return;
    };

    let group = if keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        10
    } else {
        1
    };

    let position = crosshairs
        .iter()
        .next()
        .map(Transform2D::from)
        .unwrap_or_default();

    match spawn {
        Spawn::Bonus => {
            commands.queue(BonusSpawn::new(position.position, u8::MAX));
        }
        Spawn::Human => {
            spawn_actors(&mut commands, position, &ActorConfig::HUMAN, group);
        }
        Spawn::Zombie => {
            spawn_actors(&mut commands, position, &ActorConfig::ZOMBIE, group);
        }
        Spawn::Explosion => {
            if let Some(explosion) = &ProjectileConfig::TBG_7V.explosion {
                commands.queue(Explode {
                    config: explosion,
                    position: position.position,
                    shooter: None,
                });
            }
        }
    }
}

fn spawn_actors(
    commands: &mut Commands,
    transform: Transform2D,
    config: &'static ActorConfig,
    group: u8,
) {
    for _ in 0..group {
        let entity = commands.spawn_empty().id();

        commands.queue(ActorSet {
            entity,
            config,
            position: transform.position,
            rotation: -transform.rotation,
        });

        commands.queue(ActorBotSet { entity });

        let weapon = match config.kind {
            ActorKind::Human => WeaponConfig::ALL.choose(&mut rand::thread_rng()),
            ActorKind::Zombie => None,
        };

        commands.queue(WeaponSet { entity, weapon });
    }
}

enum Spawn {
    Bonus,
    Human,
    Zombie,
    Explosion,
}

fn get_draw_queue() -> &'static Mutex<Vec<Shape>> {
    return DRAW_QUEUE.get_or_init(|| Mutex::new(Vec::new()));
}

pub fn debug_line(head: Vec2, tail: Vec2, color: Srgba) {
    if let Ok(queue) = get_draw_queue().lock().as_mut() {
        queue.push(Shape::Line(head, tail, color));
    }
}

pub fn debug_circle(center: Vec2, radius: f32, color: Srgba) {
    if let Ok(queue) = get_draw_queue().lock().as_mut() {
        queue.push(Shape::Circle(center, radius, color));
    }
}

enum Shape {
    Line(Vec2, Vec2, Srgba),
    Circle(Vec2, f32, Srgba),
}
