use crate::{
    component::{Actor, Player},
    data::WORLD_SIZE_HALF,
    model::{ActorAction, ActorActionsExt},
    resource::Config,
};
use bevy::{
    ecs::system::Query,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::{EventReader, Input, KeyCode, MouseButton, Res, Transform},
    time::Time,
};

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

    let rotation = mouse_delta_x * config.controls.mouse_sensitivity;
    let extra_rotation = rotation * Player::EXTRA_ROTATION_MULTIPLAYER;

    for (mut player, mut actor, mut transform) in query.iter_mut() {
        let player_rotation = rotation + player.add_extra_rotation(extra_rotation);

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
        transform.rotate_local_z(player_rotation);

        let limit = WORLD_SIZE_HALF;
        transform.translation.x = transform.translation.x.clamp(-limit, limit);
        transform.translation.y = transform.translation.y.clamp(-limit, limit);
    }
}
