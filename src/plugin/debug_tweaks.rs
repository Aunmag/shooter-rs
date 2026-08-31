use bevy::{
    app::Update, ecs::{resource::Resource, system::{Commands, ResMut}, world::World}, input::{ButtonInput, keyboard::KeyCode}, prelude::{App, Plugin, Res},
};
use crate::plugin::Health;

/// Allows quick tweaks in debugging process by binding any settings to `DebugTweaks` resource
/// properties
pub struct DebugTweaksPlugin;

impl Plugin for DebugTweaksPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugTweaks {
            // 70
            // tone_1: 1.1,
            // tone_2: 1.05, // 1.0 - 1.05 - 1.1?
            // delay: 0.35,
            // volume_2: 1.0, // 0.6 .. 0.8?

            // 150
            tone_1: 1.2,
            tone_2: 1.1, // 1.0 - 1.05 - 1.1?
            delay: 0.35,
            volume_2: 0.8, // 0.8 .. 1.0?
            fuzz: 0.0,
        });
        app.add_systems(Update, on_update);
    }
}

#[derive(Resource)]
pub struct DebugTweaks {
    pub tone_1: f32,
    pub tone_2: f32,
    pub delay: f32,
    pub volume_2: f32,
    pub fuzz: f32,
}

fn on_update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut tweaks: ResMut<DebugTweaks>,
    mut commands: Commands,
) {
    let mut mp = 1.0;

    if keyboard.pressed(KeyCode::ShiftLeft) {
        mp *= 2.0;
    }
    if keyboard.pressed(KeyCode::ControlLeft) {
        mp /= 2.0;
    }
    if keyboard.pressed(KeyCode::AltLeft) {
        mp /= 4.0;
    }

    if keyboard.just_pressed(KeyCode::ArrowUp) {
        tweaks.tone_1 += 0.1 * mp;
        log::debug!("tone_1={}", tweaks.tone_1);
        commands.queue(move |world: &mut World| {
            for mut health in world.query::<&mut Health>().iter_mut(world) {
                health.health += 0.1 * mp;
                health.health = health.health.clamp(0.0, 1.0);
            }
        });
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        tweaks.tone_1 -= 0.1 * mp;
        log::debug!("tone_1={}", tweaks.tone_1);
        commands.queue(move |world: &mut World| {
            for mut health in world.query::<&mut Health>().iter_mut(world) {
                health.health -= 0.1 * mp;
                health.health = health.health.clamp(0.000001, 1.0);
            }
        });
    }

    if keyboard.just_pressed(KeyCode::KeyP) {
        tweaks.tone_2 += 0.1 * mp;
        log::debug!("tone_2={}", tweaks.tone_2);
    }
    if keyboard.just_pressed(KeyCode::KeyL) {
        tweaks.tone_2 -= 0.1 * mp;
        log::debug!("tone_2={}", tweaks.tone_2);
    }

    if keyboard.just_pressed(KeyCode::ArrowRight) {
        tweaks.delay += 0.1 * mp;
        log::debug!("delay={}", tweaks.delay);
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        tweaks.delay -= 0.1 * mp;
        log::debug!("delay={}", tweaks.delay);
    }
    tweaks.delay = tweaks.delay.clamp(0.0, 1.0);

    if keyboard.just_pressed(KeyCode::Digit9) {
        tweaks.volume_2 -= 0.1 * mp;
        log::debug!("volume_2={}", tweaks.volume_2);
    }
    if keyboard.just_pressed(KeyCode::Digit0) {
        tweaks.volume_2 += 0.1 * mp;
        log::debug!("volume_2={}", tweaks.volume_2);
    }

    if keyboard.just_pressed(KeyCode::KeyZ) {
        tweaks.fuzz -= 0.01 * mp;
        log::debug!("fuzz={}", tweaks.fuzz);
    }
    if keyboard.just_pressed(KeyCode::KeyX) {
        tweaks.fuzz += 0.01 * mp;
        log::debug!("fuzz={}", tweaks.fuzz);
    }
    tweaks.fuzz = tweaks.fuzz.clamp(0.0, 1.0);
}
