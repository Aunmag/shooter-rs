use bevy::{
    app::Update,
    ecs::system::{ResMut, Resource},
    input::{keyboard::KeyCode, ButtonInput},
    prelude::{App, Plugin, Res},
};

/// Allows quick tweaks in debugging process by binding any settings to `DebugTweaks` resource
/// properties
pub struct DebugTweaksPlugin;

impl Plugin for DebugTweaksPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugTweaks { x: 0, y: 0 });
        app.add_systems(Update, on_update);
    }
}

#[derive(Resource)]
pub struct DebugTweaks {
    pub x: u8,
    pub y: u8,
}

fn on_update(keyboard: Res<ButtonInput<KeyCode>>, mut tweaks: ResMut<DebugTweaks>) {
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        tweaks.y = tweaks.y.saturating_add(1);
        log::debug!("y={}", tweaks.y);
    }

    if keyboard.just_pressed(KeyCode::ArrowDown) {
        tweaks.y = tweaks.y.saturating_sub(1);
        log::debug!("y={}", tweaks.y);
    }

    if keyboard.just_pressed(KeyCode::ArrowRight) {
        tweaks.x = tweaks.x.saturating_add(1);
        log::debug!("x={}", tweaks.x);
    }

    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        tweaks.x = tweaks.x.saturating_sub(1);
        log::debug!("x={}", tweaks.x);
    }
}
