use crate::{
    component::{Actor, Player, PlayerCrosshair},
    data::WORLD_SIZE_HALF,
    model::{ActorAction, ActorActionsExt},
    plugin::{CameraTarget, Crosshair},
    resource::Settings,
    util::ext::{TransformExt, Vec2Ext},
};
use bevy::{
    ecs::{
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query},
        world::World,
    },
    hierarchy::DespawnRecursiveExt,
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
            Entity,
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
    settings: Res<Settings>,
) {
    let mut mouse_delta_x = 0.0;

    for event in mouse_motion.read() {
        mouse_delta_x -= event.delta.x;
    }

    for (entity, mut player, mut actor, mut transform, camera) in players.iter_mut() {
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
            if let Some(crosshair) = player.crosshair.take() {
                commands.entity(crosshair.entity).despawn_recursive();

                // reset player direction
                commands.add(move |world: &mut World| {
                    for mut camera in world
                        .query_filtered::<&mut Transform, With<Camera>>()
                        .iter_mut(world)
                    {
                        camera.rotation = Quat::from_rotation_z(direction);
                    }
                });
            } else {
                commands.add(move |world: &mut World| {
                    let crosshair = Crosshair::spawn(world);

                    if let Some(mut player) = world.get_mut::<Player>(entity) {
                        player.crosshair = Some(PlayerCrosshair::new(crosshair));
                    }
                });
            }
        }

        let limit = WORLD_SIZE_HALF;
        transform.translation.x = transform.translation.x.clamp(-limit, limit);
        transform.translation.y = transform.translation.y.clamp(-limit, limit);

        if player.crosshair.is_some() {
            // TODO: optimize and simplify
            actor.movement = actor
                .movement
                .rotate_by_quat(Quat::from_rotation_z(-direction));

            if let Some(camera) = cameras.iter().next() {
                actor.movement = actor.movement.rotate_by_quat(camera.rotation);
            }
        } else {
            let rotation_base = mouse_delta_x * settings.controls.mouse_sensitivity;
            let rotation_extra = rotation_base * Player::EXTRA_ROTATION_MULTIPLAYER;
            transform.rotate_local_z(rotation_base + player.add_extra_rotation(rotation_extra));
        }

        if let Some(mut camera) = camera {
            if player.crosshair.is_some() {
                camera.sync_angle = None;
            } else {
                camera.sync_angle = Some(player.get_extra_rotation());
            }
        }
    }
}
