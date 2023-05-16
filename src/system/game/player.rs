use crate::component::Actor;
use crate::component::Player;
use crate::model::ActorAction;
use crate::model::ActorActionsExt;
use crate::resource::Config;
use bevy::ecs::system::Query;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::EventReader;
use bevy::prelude::Input;
use bevy::prelude::KeyCode;
use bevy::prelude::MouseButton;
use bevy::prelude::Res;
use bevy::prelude::Transform;
use bevy::prelude::With;
use std::f32::consts::TAU;

pub fn player(
    mut query: Query<(&mut Actor, &mut Transform), With<Player>>,
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

    for (mut actor, mut transform) in query.iter_mut() {
        actor
            .actions
            .set(ActorAction::MovementForward, keyboard.pressed(KeyCode::W));

        actor
            .actions
            .set(ActorAction::MovementBackward, keyboard.pressed(KeyCode::S));

        actor
            .actions
            .set(ActorAction::MovementLeftward, keyboard.pressed(KeyCode::A));

        actor
            .actions
            .set(ActorAction::MovementRightward, keyboard.pressed(KeyCode::D));

        actor
            .actions
            .set(ActorAction::Sprint, keyboard.pressed(KeyCode::LShift));

        actor
            .actions
            .set(ActorAction::Attack, mouse.pressed(MouseButton::Left));

        transform.rotate_local_z(rotation)
    }
}
