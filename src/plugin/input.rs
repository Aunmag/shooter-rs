use crate::resource::{Settings, WindowModeSettings};
use bevy::{
    app::{AppExit, Update},
    ecs::{query::With, system::Command, world::World},
    input::ButtonInput,
    prelude::{App, Commands, KeyCode, Plugin, Res},
    window::{CursorGrabMode, CursorOptions, PrimaryWindow, Window},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_update);
    }
}

fn on_update(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        commands.write_message(AppExit::Success);
    }

    if keyboard.just_pressed(KeyCode::Tab) {
        // TODO: hide cursor on widow click
        commands.queue(CursorGrab(false));
    }

    if keyboard.just_pressed(KeyCode::F11) {
        commands.queue(|world: &mut World| {
            let mut settings = world.resource_mut::<Settings>();

            settings.display.mode = match settings.display.mode {
                WindowModeSettings::Fullscreen => WindowModeSettings::Windowed,
                WindowModeSettings::Borderless => WindowModeSettings::Windowed,
                WindowModeSettings::Windowed => WindowModeSettings::Borderless,
            };

            let display = settings.display.clone();

            for mut window in world
                .query_filtered::<&mut Window, With<PrimaryWindow>>()
                .iter_mut(world)
            {
                window.mode = display.mode();

                if display.mode == WindowModeSettings::Windowed {
                    window
                        .resolution
                        .set(display.window_w as f32, display.window_h as f32);
                }
            }
        });
    }
}

pub struct CursorGrab(pub bool);

impl Command for CursorGrab {
    fn apply(self, world: &mut World) {
        for mut cursor in world
            .query_filtered::<&mut CursorOptions, With<PrimaryWindow>>()
            .iter_mut(world)
        {
            cursor.grab_mode = if self.0 {
                CursorGrabMode::Confined
            } else {
                CursorGrabMode::None
            };

            cursor.visible = !self.0;
        }
    }
}
