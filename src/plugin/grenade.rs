use crate::{
    plugin::{AudioPlay, AudioTracker, DirtParticleSpawn, Explode, ProjectileExplosion},
    state::AppState,
    util::ext::{AppExt, Vec2Ext},
};
use bevy::{
    app::{App, Plugin},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Command, Res},
    },
    prelude::{Commands, Query, Vec2, World},
    time::Time,
    transform::components::Transform,
};
use std::time::Duration;

pub struct GrenadePlugin;

impl Plugin for GrenadePlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update);
    }
}

#[derive(Component)]
pub struct Grenade {
    pub velocity: Vec2,
    pub distance_limit: f32,
    pub explosion: &'static ProjectileExplosion,
    pub shooter: Option<Entity>,
}

pub struct GrenadeSpawn {
    pub grenade: Grenade,
    pub image: &'static str,
}

fn on_update(
    mut query: Query<(Entity, &Grenade, &mut Transform)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (entity, grenade, mut transform) in query.iter_mut() {
        let mut traveled = grenade.velocity * time.as_secs_f32();
        let stopped = traveled.is_long(grenade.distance_limit);

        if stopped {
            traveled = grenade.velocity.normalize() * grenade.distance_limit;
        }

        let position = transform.translation.truncate() + traveled;
        transform.translation.x = position.x;
        transform.translation.y = position.y;
        // TODO: update scale

        if stopped {
            let shooter = grenade.shooter;
            let explosion = grenade.explosion;

            commands.queue(move |world: &mut World| {
                world.entity_mut(entity).despawn();

                world.resource::<AudioTracker>().queue(AudioPlay {
                    path: "sounds/bullet/ground".into(),
                    volume: 0.8,
                    falloff: AudioPlay::FALLOFF_SHORTER,
                    source: Some(position),
                    ..AudioPlay::DEFAULT
                });

                DirtParticleSpawn {
                    amount: 8,
                    position,
                    velocity_min: 0.3,
                    velocity_max: 1.0,
                    duration: Duration::from_millis(250),
                    size_max: 1.2,
                }
                .apply(world);

                Explode {
                    config: explosion,
                    position,
                    shooter,
                }
                .apply(world);
            });
        }
    }
}
