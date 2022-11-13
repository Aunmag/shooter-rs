use crate::component::Actor;
use crate::component::ActorActions;
use crate::component::Player;
use crate::resource::Config;
use bevy::ecs::system::Query;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::EventReader;
use bevy::prelude::Input;
use bevy::prelude::KeyCode;
use bevy::prelude::MouseButton;
use bevy::prelude::Res;
use bevy::prelude::With;
use std::f32::consts::TAU;

pub fn player(
    mut query: Query<&mut Actor, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    config: Res<Config>,
) {
    let mut mouse_delta_x = 0.0;

    for event in mouse_motion.iter() {
        mouse_delta_x -= event.delta.x;
    }

    let rotation = (mouse_delta_x * config.controls.mouse_sensitivity) % TAU;

    for mut actor in query.iter_mut() {
        actor
            .actions
            .set(ActorActions::MOVEMENT_FORWARD, keyboard.pressed(KeyCode::W));

        actor.actions.set(
            ActorActions::MOVEMENT_BACKWARD,
            keyboard.pressed(KeyCode::S),
        );

        actor.actions.set(
            ActorActions::MOVEMENT_LEFTWARD,
            keyboard.pressed(KeyCode::A),
        );

        actor.actions.set(
            ActorActions::MOVEMENT_RIGHTWARD,
            keyboard.pressed(KeyCode::D),
        );

        actor
            .actions
            .set(ActorActions::ATTACK, mouse.pressed(MouseButton::Left));

        actor.rotation = rotation;
    }
}
