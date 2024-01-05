use crate::{
    command::{ActorRelease, BloodSpawn},
    component::{Actor, Health, Player},
    event::ActorDeathEvent,
    model::AudioPlay,
    resource::{AudioTracker, Config, GameMode},
};
use bevy::{
    ecs::{
        query::Has,
        system::{Query, Res, ResMut},
    },
    math::Vec3Swizzles,
    prelude::{Commands, DespawnRecursiveExt, Entity, EventWriter, Transform},
};

pub fn health(
    mut query: Query<(Entity, &Actor, &mut Health, &Transform, Has<Player>)>,
    mut death_events: EventWriter<ActorDeathEvent>,
    mut audio: ResMut<AudioTracker>,
    mut commands: Commands,
    config: Res<Config>,
) {
    for (entity, actor, mut health, transform, is_player) in query.iter_mut() {
        let actor = actor.config;
        let point = transform.translation.xy();
        let damage = health.get_damage_normalized();

        if health.is_alive() && damage > actor.pain_threshold {
            audio.queue(AudioPlay {
                path: format!("{}/pain", actor.get_assets_path()).into(),
                volume: 0.9,
                source: Some(point),
                ..AudioPlay::DEFAULT
            });
        }

        if health.is_just_died() {
            if config.game.modes.contains(&GameMode::Bench) {
                health.heal();
            } else {
                audio.queue(AudioPlay {
                    path: format!("{}/death", actor.get_assets_path()).into(),
                    volume: 1.0,
                    source: Some(point),
                    ..AudioPlay::DEFAULT
                });

                commands.add(ActorRelease(entity));

                death_events.send(ActorDeathEvent {
                    kind: actor.kind,
                    position: point,
                    is_player,
                });

                commands.entity(entity).despawn_recursive();
            }
        }

        if let Some(blood) = BloodSpawn::new(point, damage) {
            commands.add(blood);
        }

        if health.is_just_died() {
            if let Some(blood) = BloodSpawn::new(point, 0.75) {
                commands.add(blood);
            }
        }

        health.commit();
    }
}
