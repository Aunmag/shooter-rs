mod camera;
mod control;
mod edge;
mod node;
mod ctrl;
mod util;

use crate::{
    plugin::{CursorGrab, editor::{camera::EditorCameraPlugin, control::ControlPlugin, edge::Edge, node::Node}}, state::AppState, util::ext::AppExt,
};
use bevy::{
    app::Update, ecs::world::Command, prelude::{App, Plugin, World},
};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EditorCameraPlugin);
        app.add_plugins(ControlPlugin);
        app.add_state_system_enter(AppState::Game, on_enter); // TODO: not only game enter
        app.add_systems(Update, Node::on_render);
        app.add_systems(Update, Edge::on_render);
    }
}

fn on_enter(world: &mut World) {
    EditorCameraPlugin::spawn_camera(world);
    CursorGrab(false).apply(world);
}
