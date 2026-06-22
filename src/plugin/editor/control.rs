use crate::plugin::{
    editor::{edge::Edge, node::Node},
    MainCamera,
};
use bevy::{
    app::{App, Update},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
        world::World,
    },
    hierarchy::DespawnRecursiveExt,
    input::{keyboard::KeyCode, mouse::MouseButton, ButtonInput},
    math::Vec2,
    prelude::Plugin,
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::{PrimaryWindow, Window},
};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_update);
    }
}

// TODO: move to other struct?
#[derive(Component)]
pub struct Selection;

impl Selection {
    pub fn select(world: &mut World, entity: Entity, add: bool) {
        if !add {
            Self::deselect_all(world);
        }

        world.entity_mut(entity).insert(Self);
    }

    pub fn deselect_all(world: &mut World) {
        for entity in world
            .query_filtered::<Entity, With<Self>>()
            .iter(world)
            .collect::<Vec<_>>()
        {
            world.entity_mut(entity).remove::<Self>();
        }
    }

    pub fn delete_selected(world: &mut World) {
        for entity in world
            .query_filtered::<Entity, With<Self>>()
            .iter(world)
            .collect::<Vec<_>>()
        {
            world.entity_mut(entity).despawn_recursive();
        }
    }
}

fn on_update(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    let mut cursor = None;

    if let Some(cursor_on_screen) = windows.iter().next().and_then(|w| w.cursor_position()) {
        cursor = cameras
            .iter()
            .next()
            .and_then(|(c, t)| c.viewport_to_world(t, cursor_on_screen))
            .map(|v| v.origin.truncate());
    }

    // TODO: chain?
    let Some(cursor) = cursor else {
        return;
    };

    if keyboard.just_pressed(KeyCode::Delete) {
        commands.add(Selection::delete_selected);
        return; // TODO: return?
    }

    if keyboard.just_pressed(KeyCode::KeyF) {
        commands.add(connect_nodes);
        return; // TODO: return?
    }

    if mouse.just_pressed(MouseButton::Left) {
        let add = keyboard.pressed(KeyCode::ShiftLeft);
        commands.add(move |w: &mut World| select_node_at(w, cursor, add));
        return; // TODO: return?
    }

    if keyboard.just_pressed(KeyCode::KeyE) {
        let connect = keyboard.pressed(KeyCode::ShiftLeft);
        commands.add(move |w: &mut World| create_node(w, cursor, connect));
        return; // TODO: return?
    }

    // TODO: relative scale by mouse movement
    // TODO: make as system
    if keyboard.pressed(KeyCode::KeyR) {
        commands.add(move |w: &mut World| scale_node(w, cursor));
        return; // TODO: return?
    }
}

fn create_node(world: &mut World, position: Vec2, connect: bool) {
    let entity = world
        .spawn(Node {
            p: position,
            r: 0.1,
        })
        .id();

    Selection::select(world, entity, connect);

    if connect {
        connect_nodes(world);
    }
}

fn select_node_at(world: &mut World, position: Vec2, add: bool) {
    let mut closest_distance = f32::INFINITY;
    let mut closest = None;

    for (entity, node) in world.query::<(Entity, &Node)>().iter(world) {
        let distance = position.distance_squared(node.p);

        if distance >= node.r * node.r {
            continue;
        }

        if distance >= closest_distance {
            continue;
        }

        closest_distance = distance;
        closest = Some(entity);
    }

    if let Some(entity) = closest {
        Selection::select(world, entity, add);
    } else if !add {
        Selection::deselect_all(world);
    }
}

fn connect_nodes(world: &mut World) {
    let mut previous = None;
    let mut edges = Vec::new();

    for entity in world
        .query_filtered::<Entity, With<Selection>>()
        .iter(world)
    {
        if let Some(previous) = previous {
            edges.push((previous, entity));
        }

        previous = Some(entity);
    }

    // TODO: do not create duplicates
    for (a, b) in edges {
        world.spawn(Edge { a, b });
    }
}

fn scale_node(world: &mut World, cursor: Vec2) {
    if let Some(mut node) = world
        .query_filtered::<&mut Node, With<Selection>>()
        .iter_mut(world)
        .next()
    {
        node.r = node.p.distance(cursor);
    }
}
