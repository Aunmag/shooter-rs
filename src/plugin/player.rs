use crate::{
    component::Actor,
    data::{LAYER_ACTOR_PLAYER, WORLD_SIZE_HALF},
    model::{ActorAction, ActorActionsExt, AppState},
    plugin::{camera_target::CameraTarget, kinetics::Kinetics, Crosshair, Health, StatusBar},
    resource::Settings,
    util::ext::{AppExt, TransformExt, Vec2Ext},
};
use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Command, Query},
    },
    hierarchy::DespawnRecursiveExt,
    input::mouse::MouseMotion,
    math::{Quat, Vec2},
    prelude::{
        App, Commands, EventReader, Input, KeyCode, MouseButton, Plugin, Res, Transform, World,
    },
    render::camera::Camera,
};
use std::f32::consts::FRAC_PI_2;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update);
    }
}

#[derive(Component)]
pub struct Player {
    pub is_controllable: bool,
    pub crosshair: Option<PlayerCrosshair>,
    extra_rotation: f32,
}

impl Player {
    pub const EXTRA_ROTATION_MULTIPLAYER: f32 = 0.1;
    pub const EXTRA_ROTATION_MAX: f32 = 0.11;

    pub fn new(is_controllable: bool) -> Self {
        return Self {
            is_controllable,
            crosshair: None,
            extra_rotation: 0.0,
        };
    }

    pub fn add_extra_rotation(&mut self, value: f32) -> f32 {
        let previous = self.extra_rotation;
        let limit = Self::EXTRA_ROTATION_MAX;
        self.extra_rotation = (self.extra_rotation + value).clamp(-limit, limit);
        let added = self.extra_rotation - previous;
        return added;
    }

    pub fn get_extra_rotation(&self) -> f32 {
        return self.extra_rotation;
    }
}

pub struct PlayerCrosshair {
    pub entity: Entity,
    pub distance: f32,
}

// TODO: separate aim?
pub fn on_update(
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
                        player.crosshair = Some(PlayerCrosshair {
                            entity: crosshair,
                            distance: 1.0,
                        });
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

pub struct PlayerSet {
    pub entity: Entity,
    pub is_controllable: bool,
}

impl Command for PlayerSet {
    fn apply(self, world: &mut World) {
        let health_multiplier = 1.0 / world.resource::<Settings>().game.difficulty;

        if let Some(mut actor) = world.get_mut::<Actor>(self.entity) {
            actor.skill = 1.0; // to keep game balance well, player skill must always be 1.0
        }

        if let Some(mut health) = world.get_mut::<Health>(self.entity) {
            health.multiply_resistance(health_multiplier);
        }

        if let Some(mut transform) = world.get_mut::<Transform>(self.entity) {
            transform.translation.z = LAYER_ACTOR_PLAYER;
        }

        if let Some(mut kinetics) = world.get_mut::<Kinetics>(self.entity) {
            kinetics.drag = Kinetics::DRAG_PLAYER;
        }

        world
            .entity_mut(self.entity)
            .insert(Player::new(self.is_controllable))
            .insert(CameraTarget::default());

        StatusBar::spawn(world, self.entity);
    }
}
