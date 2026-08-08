use crate::{
    data::{LAYER_ACTOR_PLAYER, WORLD_SIZE_HALF},
    plugin::{
        camera::MainCamera, camera_target::CameraTarget, kinetics::Kinetics, Actor, ActorAction,
        ActorActions, ActorActionsExt, ActorConfig, ActorSet, Crosshair, Health, StatusBar,
        WeaponConfig, WeaponSet,
    },
    resource::Settings,
    state::AppState,
    util::ext::{AppExt, QuatExt, Vec2Ext},
};
use bevy::{
    camera::Camera,
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        schedule::{IntoScheduleConfigs, SystemSet},
        system::{Command, In, IntoSystem, Query},
    },
    input::{mouse::MouseMotion, ButtonInput},
    math::Vec2,
    prelude::{App, KeyCode, MessageReader, MouseButton, Plugin, Res, Transform, World},
    transform::components::GlobalTransform,
};

const EXTRA_ROTATION_MULTIPLAYER: f32 = 0.1;
const EXTRA_ROTATION_MAX: f32 = 0.11;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSystems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(
            AppState::Game,
            (on_update_1.pipe(on_update_2)).in_set(PlayerSystems),
        );
    }
}

#[derive(Component)]
pub struct Player {
    pub is_controllable: bool, // TODO: avoid
    is_aiming: bool,
    extra_rotation: f32,
}

impl Player {
    fn rotate(&mut self, value: f32) -> f32 {
        let limit = EXTRA_ROTATION_MAX;
        let extra_rotation_before = self.extra_rotation;
        self.extra_rotation += value * EXTRA_ROTATION_MULTIPLAYER;
        self.extra_rotation = self.extra_rotation.clamp(-limit, limit);
        let extra_rotation_change = self.extra_rotation - extra_rotation_before;
        return value + extra_rotation_change;
    }
}

fn on_update_1(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
) -> Input {
    let mut input = Input::default();

    for event in mouse_motion.read() {
        input.mouse_motion += event.delta;
    }

    if keyboard.pressed(KeyCode::KeyW) {
        input.movement.x += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        input.movement.x -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        input.movement.y += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        input.movement.y -= 1.0;
    }

    input
        .actions
        .set(ActorAction::Sprint, keyboard.pressed(KeyCode::ShiftLeft));

    input
        .actions
        .set(ActorAction::Attack, mouse.pressed(MouseButton::Left));

    input
        .actions
        .set(ActorAction::Reload, keyboard.pressed(KeyCode::KeyR));

    input.actions.set(
        ActorAction::AimToggle,
        mouse.just_pressed(MouseButton::Right),
    );

    return input;
}

fn on_update_2(
    In(input): In<Input>,
    mut players: Query<
        (
            &mut Player,
            &mut Actor,
            &mut Transform,
            Option<&mut CameraTarget>,
        ),
        Without<MainCamera>,
    >,
    cameras: Query<(&Camera, &Transform, &GlobalTransform), With<MainCamera>>,
    settings: Res<Settings>,
) {
    let camera = cameras.iter().next();

    for (mut player, mut actor, mut transform, camera_target) in players.iter_mut() {
        let limit = WORLD_SIZE_HALF;
        transform.translation.x = transform.translation.x.clamp(-limit, limit);
        transform.translation.y = transform.translation.y.clamp(-limit, limit);

        if !player.is_controllable {
            continue;
        }

        actor.movement = input.movement;
        actor.actions = input.actions;

        if actor.actions.contains(ActorAction::AimToggle) {
            player.is_aiming = !player.is_aiming;

            if !player.is_aiming {
                // sync player back with camera
                if let Some(camera) = camera {
                    transform.rotation = camera.1.rotation.perp();
                    player.extra_rotation = 0.0;
                }
            }
        }

        if let Some((camera, _, camera_transform_global)) = camera {
            update_aim(
                &mut actor,
                &mut transform,
                &mut player,
                camera,
                camera_transform_global,
                input.mouse_motion,
                settings.controls.mouse_sensitivity,
            );
        }

        if player.is_aiming {
            // make movement relative to the camera
            actor.movement.y = -actor.movement.y;
            actor.movement = actor.movement.rotate_by_quat(transform.rotation);
            let x_copy = actor.movement.x;
            actor.movement.x = actor.movement.y;
            actor.movement.y = x_copy;

            if let Some(camera) = camera {
                actor.movement = actor.movement.rotate_by_quat(camera.1.rotation);
            }
        } else {
            actor.aim_distance = f32::max(actor.aim_distance, 1.0);
        }

        if let Some(mut camera_target) = camera_target {
            if player.is_aiming {
                camera_target.sync_angle = None;
            } else {
                camera_target.sync_angle = Some(player.extra_rotation);
            }
        }
    }
}

fn update_aim(
    actor: &mut Actor,
    actor_transform: &mut Transform,
    player: &mut Player,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    mouse_motion: Vec2,
    mouse_sensitivity: f32,
) {
    if mouse_motion.is_zero() {
        return; // early return to prevent floating point error grow
    }

    if !player.is_aiming {
        actor_transform.rotate_local_z(player.rotate(-mouse_motion.x * mouse_sensitivity));

        if mouse_motion.y == 0.0 {
            return; // early return to prevent floating point error grow
        }
    }

    // aim must in sync with player while it moves, also player direction can be changed because of
    // weapon recoil, so aim should be affected too
    let position = actor_transform.translation.truncate();
    let on_world_old = position + actor_transform.rotation.as_vec() * actor.aim_distance;

    let Ok(on_screen_old) = camera.world_to_viewport(camera_transform, on_world_old.extend(0.0))
    else {
        return;
    };

    let mut on_screen_new = on_screen_old + mouse_motion;

    // clamp aim inside view port
    if let Some(viewport_size) = camera.logical_viewport_size() {
        on_screen_new.x = on_screen_new.x.clamp(0.0, viewport_size.x);
        on_screen_new.y = on_screen_new.y.clamp(0.0, viewport_size.y);
    }

    // put aim to it's updated position
    let Ok(on_world_new) = camera
        .viewport_to_world(camera_transform, on_screen_new)
        .map(|v| v.origin.truncate())
    else {
        return;
    };

    // update only when cursor moved more than 1px actually, otherwise errors may grow
    if (on_screen_new - on_screen_old).is_long(0.99) {
        actor.aim_distance = position.distance(on_world_new);
    }

    if player.is_aiming {
        actor_transform.rotation = (on_world_new - position).as_quat();
    }
}

pub struct PlayerSet {
    pub entity: Entity,
    pub is_controllable: bool,
}

impl Command for PlayerSet {
    type Out = ();

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

        Crosshair::spawn(world, self.entity);

        world
            .entity_mut(self.entity)
            .insert(Player {
                is_controllable: self.is_controllable,
                is_aiming: false,
                extra_rotation: 0.0,
            })
            .insert(CameraTarget::default());

        StatusBar::spawn(world, self.entity);
    }
}

pub struct PlayerSpawn {
    pub config: &'static ActorConfig,
    pub weapon: &'static WeaponConfig,
    pub is_controllable: bool,
}

impl Command for PlayerSpawn {
    type Out = ();

    fn apply(self, world: &mut World) {
        let entity = world.spawn_empty().id();

        ActorSet {
            entity,
            config: self.config,
            position: Vec2::ZERO,
            rotation: 0.0,
        }
        .apply(world);

        PlayerSet {
            entity,
            is_controllable: self.is_controllable,
        }
        .apply(world);

        WeaponSet {
            entity,
            weapon: Some(self.weapon),
        }
        .apply(world);
    }
}

#[derive(Default)]
struct Input {
    mouse_motion: Vec2,
    movement: Vec2,
    actions: ActorActions,
}
