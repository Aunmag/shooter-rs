use crate::resource::{Settings, WindowModeSettings};
use bevy::{
    app::{AppExit, Update},
    ecs::{
        query::With,
        world::{Command, World},
    },
    input::ButtonInput,
    prelude::{App, Commands, KeyCode, Plugin, Res},
    window::{CursorGrabMode, PrimaryWindow, Window},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_update);
    }
}

fn on_update(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        commands.add(|w: &mut World| {
            w.send_event(AppExit::Success);
        });
    }

    if keyboard.just_pressed(KeyCode::Tab) {
        // TODO: hide cursor on widow click
        commands.add(CursorGrab(false));
    }

    if keyboard.just_pressed(KeyCode::F11) {
        commands.add(|world: &mut World| {
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
                        .set(f32::from(display.window_w), f32::from(display.window_h));
                }
            }
        });
    }
}

pub struct CursorGrab(pub bool);

impl Command for CursorGrab {
    fn apply(self, world: &mut World) {
        for mut window in world
            .query_filtered::<&mut Window, With<PrimaryWindow>>()
            .iter_mut(world)
        {
            window.cursor.grab_mode = if self.0 {
                CursorGrabMode::Confined
            } else {
                CursorGrabMode::None
            };

            window.cursor.visible = !self.0;
        }
    }
}
