use crate::{command::CursorGrab, resource::Settings};
use bevy::{
    app::AppExit,
    ecs::{query::With, world::World},
    input::ButtonInput,
    prelude::{Commands, KeyCode, Res},
    window::{PrimaryWindow, Window},
};

pub fn input(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
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
            settings.display.full_screen = !settings.display.full_screen;
            settings.clone().save_in_background();
            let display = settings.display.clone();

            for mut window in world
                .query_filtered::<&mut Window, With<PrimaryWindow>>()
                .iter_mut(world)
            {
                window.mode = display.mode();

                if !display.full_screen {
                    window.resolution.set(
                        f32::from(display.window_size_x),
                        f32::from(display.window_size_y),
                    );
                }
            }
        });
    }
}
