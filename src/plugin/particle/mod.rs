mod flesh;
mod shell;

pub use self::{flesh::*, shell::*};
use crate::{
    component::Player,
    data::{LAYER_GROUND, LAYER_PROJECTILE, TRANSFORM_SCALE},
    model::AppState,
    util::{
        ext::{AppExt, DurationExt},
        math::interpolate,
    },
};
use bevy::{
    app::{App, Plugin},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    input::{keyboard::KeyCode, Input},
    math::{Quat, Vec2, Vec3},
    prelude::{Res, Time, Transform},
};
use std::{
    f32::consts::{PI, TAU},
    time::Duration,
};

const DEBUG: bool = false;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update);

        if DEBUG {
            app.add_state_system(AppState::Game, on_update_debug);
        }
    }
}

#[derive(Component)]
struct Particle {
    config: &'static ParticleConfig,
    position: Vec2,
    rotation: f32,
    velocity: Vec2,
    velocity_spin: Vec3,
    since: Duration,
    until: Duration,
    scale: f32,
}

struct ParticleConfig {
    jump_factor: f32,
    on_destroy: fn(Entity, Vec2, &mut Commands) -> (),
}

fn on_update(
    mut query: Query<(Entity, &Particle, &mut Transform)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let now = time.elapsed();

    for (entity, particle, mut transform) in query.iter_mut() {
        let progress = now.progress(particle.since, particle.until);
        let calm_down = progress.powf(0.5); // emulate speed decrease over time
        let position = particle.position + particle.velocity * calm_down;
        let rotation = particle.rotation + particle.velocity_spin.z * calm_down;
        let jump = f32::sin(progress * PI); // emulate jump over Z axis from 0 to 1 and back to 0

        transform.translation.x = position.x;
        transform.translation.y = position.y;
        // TODO: player -> projectile -> ground layers
        transform.translation.z = interpolate(LAYER_GROUND, LAYER_PROJECTILE, jump);
        transform.rotation = Quat::from_rotation_z(rotation);
        transform.scale =
            TRANSFORM_SCALE * particle.scale * (1.0 + jump * particle.config.jump_factor);

        if progress < 1.0 {
            // Emulate 3D rotation by changing the scale. Note that when particle lands the ground
            // its rotation (excepting Z axis) should be reset: facing the camera straitly
            transform.scale.x *= f32::cos(calm_down * TAU * particle.velocity_spin.x);
            transform.scale.y *= f32::cos(calm_down * TAU * particle.velocity_spin.y);
        } else {
            commands.entity(entity).remove::<Particle>();
            (particle.config.on_destroy)(entity, transform.translation.truncate(), &mut commands);
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
