use crate::command::CursorGrab;
use bevy::{
    app::AppExit,
    prelude::{Commands, EventWriter, Input, KeyCode, Res},
};

pub fn input(
    mut commands: Commands,
    mut events: EventWriter<AppExit>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        events.send(AppExit);
    }

    if keyboard.just_pressed(KeyCode::Tab) {
        // TODO: hide cursor on widow click
        commands.add(CursorGrab(false));
    }
}
