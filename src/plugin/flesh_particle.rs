use super::{BloodSpawn, TileBlend};
use crate::{
    component::{Inertia, Player},
    data::{LAYER_GROUND, LAYER_HIDDEN, LAYER_PROJECTILE, TRANSFORM_SCALE},
    model::AppState,
    util::{
        ext::{AppExt, DurationExt, RngExt, Vec2Ext},
        math::interpolate,
    },
};
use bevy::{
    app::{App, Plugin},
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Command, Commands, Query},
        world::World,
    },
    input::{keyboard::KeyCode, Input},
    math::{Quat, Vec2, Vec3},
    prelude::{Res, Time, Transform},
    sprite::{Sprite, SpriteBundle},
};
use rand::Rng;
use std::{
    f32::consts::{PI, TAU},
    time::Duration,
};

const VELOCITY_MIN: f32 = 1.0;
const VELOCITY_MAX: f32 = 3.0;
const VELOCITY_SPIN: f32 = 2.5;
const DURATION: Duration = Duration::from_millis(400);
const DEBUG: bool = false;

pub struct FleshParticlePlugin;

impl Plugin for FleshParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update);

        if DEBUG {
            app.add_state_system(AppState::Game, on_update_debug);
        }
    }
}

pub struct FleshParticleSpawn(pub Entity);

impl Command for FleshParticleSpawn {
    fn apply(self, world: &mut World) {
        let now = world.resource::<Time>().elapsed();
        let mut rng = rand::thread_rng();

        let Some(position) = world
            .get::<Transform>(self.0)
            .map(|t| t.translation.truncate())
        else {
            return;
        };

        // TODO: find available automatically
        let path = format!("particle/flesh_{}.png", rng.gen_range(0..=5));
        let Some(texture) = world.resource::<AssetServer>().get_handle(path) else {
            return;
        };

        let mut velocity = Vec2::from_length(
            rng.gen_range(VELOCITY_MIN..VELOCITY_MAX),
            rng.gen_range(0.0..TAU),
        );

        if let Some(inertia) = world.get::<Inertia>(self.0) {
            velocity += inertia.velocity / 2.0;
        }

        world
            .spawn(SpriteBundle {
                sprite: Sprite {
                    flip_x: rng.gen(),
                    flip_y: rng.gen(),
                    ..Default::default()
                },
                transform: Transform {
                    translation: position.extend(LAYER_HIDDEN),
                    scale: Vec3::ZERO,
                    ..Default::default()
                },
                texture,
                ..Default::default()
            })
            .insert(Flesh {
                position,
                rotation: rng.gen_range(0.0..TAU),
                velocity,
                velocity_spin: Vec3::new(
                    rng.gen_range(0.0..VELOCITY_SPIN),
                    rng.gen_range(0.0..VELOCITY_SPIN),
                    rng.fuzz(VELOCITY_SPIN),
                ),
                since: now,
                until: now + rng.fuzz_duration(DURATION),
                scale: rng.fuzz(1.0),
            });
    }
}

#[derive(Component)]
struct Flesh {
    position: Vec2,
    rotation: f32,
    velocity: Vec2,
    velocity_spin: Vec3,
    since: Duration,
    until: Duration,
    scale: f32,
}

fn on_update(
    mut query: Query<(Entity, &Flesh, &mut Transform)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let now = time.elapsed();

    for (entity, flesh, mut transform) in query.iter_mut() {
        let progress = now.progress(flesh.since, flesh.until);
        let calm_down = progress.powf(0.5); // emulate speed decrease over time
        let position = flesh.position + flesh.velocity * calm_down;
        let rotation = flesh.rotation + flesh.velocity_spin.z * calm_down;
        let jump = f32::sin(progress * PI); // emulate jump over Z axis from 0 to 1 and back to 0

        transform.translation.x = position.x;
        transform.translation.y = position.y;
        transform.translation.z = interpolate(LAYER_GROUND, LAYER_PROJECTILE, jump);
        transform.rotation = Quat::from_rotation_z(rotation);
        transform.scale = TRANSFORM_SCALE * flesh.scale * (1.0 + jump);

        if progress < 1.0 {
            // Emulate 3D rotation by changing the scale. Note that when particle lands the ground
            // its rotation (excepting Z axis) should be reset: facing the camera straitly
            transform.scale.x *= f32::sin(calm_down * PI * flesh.velocity_spin.x);
            transform.scale.y *= f32::sin(calm_down * PI * flesh.velocity_spin.y);
        } else {
            commands.entity(entity).remove::<Flesh>();

            if let Some(blood) = BloodSpawn::new(transform.translation.truncate(), 0.2) {
                commands.add(blood);
            }

            commands.add(TileBlend::Entity(entity));
            // TODO: play land sound
        }
    }
}

fn on_update_debug(
    players: Query<Entity, With<Player>>,
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let Some(entity) = players.iter().next() else {
        return;
    };

    commands.add(FleshParticleSpawn(entity));
}
