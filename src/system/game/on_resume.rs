use crate::command::CursorLock;
use bevy::prelude::Commands;

pub fn on_resume(mut commands: Commands) {
    commands.add(CursorLock(false));
}
