use crate::{
    component::{Actor, Player},
    data::WORLD_SIZE_HALF,
    model::{ActorAction, ActorActionsExt},
    plugin::CameraTarget,
    resource::Config,
    util::ext::{TransformExt, Vec2Ext},
};
use bevy::{
    ecs::{
        query::{With, Without},
        system::{Commands, Query},
        world::World,
    },
    input::mouse::MouseMotion,
    math::{Quat, Vec2},
    prelude::{EventReader, Input, KeyCode, MouseButton, Res, Transform},
    render::camera::Camera,
};
use std::f32::consts::FRAC_PI_2;

// TODO: separate aim?
pub fn player(
    mut players: Query<
        (
            &mut Player,
            &mut Actor,
            &mut Transform,
            Option<&mut CameraTarget>,
        ),
        Without<Camera>,
    >,
    cameras: Query<&Transform, With<Camera>>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut commands: Commands,
    config: Res<Config>,
) {
    let mut mouse_delta_x = 0.0;

    for event in mouse_motion.read() {
        mouse_delta_x -= event.delta.x;
    }

    for (mut player, mut actor, mut transform, camera) in players.iter_mut() {
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

        let direction = transform.direction() - FRAC_PI_2;

        if mouse.just_pressed(MouseButton::Right) {
            player.is_aiming = !player.is_aiming;

            if player.is_aiming {
                commands.add(move |world: &mut World| {
                    for mut camera in world
                        .query_filtered::<&mut Transform, With<Camera>>()
                        .iter_mut(world)
                    {
                        camera.rotation = Quat::from_rotation_z(direction);
                    }
                });
            }
        }

        let limit = WORLD_SIZE_HALF;
        transform.translation.x = transform.translation.x.clamp(-limit, limit);
        transform.translation.y = transform.translation.y.clamp(-limit, limit);

        if !player.is_aiming {
            let rotation_base = mouse_delta_x * config.controls.mouse_sensitivity;
            let rotation_extra = rotation_base * Player::EXTRA_ROTATION_MULTIPLAYER;
            transform.rotate_local_z(rotation_base + player.add_extra_rotation(rotation_extra));
        } else {
            // TODO: optimize and simplify
            actor.movement = actor
                .movement
                .rotate_by_quat(Quat::from_rotation_z(-direction));

            if let Some(camera) = cameras.iter().next() {
                actor.movement = actor.movement.rotate_by_quat(camera.rotation);
            }
        }

        if let Some(mut camera) = camera {
            if player.is_aiming {
                camera.sync_angle = None;
            } else {
                camera.sync_angle = Some(player.get_extra_rotation());
            }
        }
    }
}
