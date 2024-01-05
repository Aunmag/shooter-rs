use crate::{
    component::{Actor, Player},
    data::WORLD_SIZE_HALF,
    model::{ActorAction, ActorActionsExt},
    resource::Config,
};
use bevy::{
    ecs::system::Query,
    input::mouse::{MouseMotion, MouseWheel},
    math::Vec2,
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

    let mut mouse_delta_x = 0.0;
    let mut zoom = 0.0;

    for event in mouse_motion.read() {
        mouse_delta_x -= event.delta.x;
    }

    for event in mouse_scroll.read() {
        zoom += event.y;
    }

    let rotation = mouse_delta_x * config.controls.mouse_sensitivity;
    let extra_rotation = rotation * Player::EXTRA_ROTATION_MULTIPLAYER;

    for (mut player, mut actor, mut transform) in query.iter_mut() {
        if !player.is_controllable {
            continue;
        }

        actor.movement = Vec2::ZERO;

        if keyboard.pressed(KeyCode::W) {
            actor.movement.x += 1.0;
        }

        if keyboard.pressed(KeyCode::S) {
            actor.movement.x -= 1.0;
        }

        if keyboard.pressed(KeyCode::A) {
            actor.movement.y += 1.0;
        }

        if keyboard.pressed(KeyCode::D) {
            actor.movement.y -= 1.0;
        }

        actor
            .actions
            .set(ActorAction::Sprint, keyboard.pressed(KeyCode::ShiftLeft));

        actor
            .actions
            .set(ActorAction::Attack, mouse.pressed(MouseButton::Left));

        actor
            .actions
            .set(ActorAction::Reload, keyboard.pressed(KeyCode::R));

        player.add_zoom(zoom);
        player.update(delta);
        transform.rotate_local_z(rotation + player.add_extra_rotation(extra_rotation));

        let limit = WORLD_SIZE_HALF;
        transform.translation.x = transform.translation.x.clamp(-limit, limit);
        transform.translation.y = transform.translation.y.clamp(-limit, limit);
    }
}
