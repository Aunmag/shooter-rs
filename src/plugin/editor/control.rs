use crate::plugin::{
    MainCamera, editor::{ctrl::{cursor::Cursor, grab::Grab, select::Select}, edge::Edge, node::Node},
};
use bevy::{
    app::{App, Update}, ecs::{
        component::Component, entity::Entity, query::With, system::{Commands, Query, Res, Resource}, world::World,
    }, hierarchy::DespawnRecursiveExt, input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton}, math::Vec2, prelude::Plugin, render::camera::Camera, transform::components::GlobalTransform, window::{PrimaryWindow, Window},
};
use bevy::ecs::schedule::IntoSystemConfigs;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>();
        app.add_systems(Update, Cursor::system);
        app.add_systems(Update, Grab::system);
        app.add_systems(Update, Select::system);
        app.add_systems(Update, on_update);
    }
}

fn on_update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    mut commands: Commands,
) {
    let Some(cursor) = cursor.p else {
        return;
    };

    if keyboard.just_pressed(KeyCode::Delete) {
        commands.add(Select::delete_selected);
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

    if keyboard.just_pressed(KeyCode::KeyG) {
        commands.add(move |w: &mut World| {
            Grab::apply_to_selected(w);
        });
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

    Select::insert(world, entity, connect);

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
        Select::insert(world, entity, add);
    } else if !add {
        Select::remove_all(world);
    }
}

fn connect_nodes(world: &mut World) {
    let mut previous = None;
    let mut edges = Vec::new();

    for entity in world
        .query_filtered::<Entity, With<Select>>()
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
        .query_filtered::<&mut Node, With<Select>>()
        .iter_mut(world)
        .next()
    {
        node.r = node.p.distance(cursor);
    }
}
