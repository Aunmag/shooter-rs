use crate::command::CursorGrab;
use bevy::prelude::Commands;

pub fn on_resume(mut commands: Commands) {
    commands.add(CursorGrab(false));
}
