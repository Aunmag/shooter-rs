use crate::{
    component::{Actor, Player},
    model::{ActorAction, ActorActionsExt},
    resource::Config,
};
use bevy::{
    ecs::system::Query,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::{EventReader, Input, KeyCode, MouseButton, Res, Transform},
    time::Time,
};
use std::f32::consts::TAU;

pub fn player(
    mut query: Query<(&mut Player, &mut Actor, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_scroll: EventReader<MouseWheel>,
    time: Res<Time>,
    config: Res<Config>,
) {
    let delta = time.delta_seconds();
    let time = time.elapsed();

    let mut mouse_delta_x = 0.0;
    let mut zoom = 0.0;

    for event in mouse_motion.iter() {
        mouse_delta_x -= event.delta.x;
    }

    for event in mouse_scroll.iter() {
        zoom += event.y;
    }

    let rotation = (mouse_delta_x * config.controls.mouse_sensitivity) % TAU;

    for (mut player, mut actor, mut transform) in query.iter_mut() {
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

        actor
            .actions
            .set(ActorAction::Reload, keyboard.pressed(KeyCode::R));

        player.add_zoom(zoom, time);
        player.update(time, delta);
        transform.rotate_local_z(rotation);
    }
}
