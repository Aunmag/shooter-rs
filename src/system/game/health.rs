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

const BLOOD_FACTOR_ON_DAMAGE: f32 = 24.0;
const BLOOD_FACTOR_ON_DEATH: f32 = 16.0;

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
        let mut blood = actor.radius * damage * BLOOD_FACTOR_ON_DAMAGE;

        if health.is_alive() && damage > actor.pain_threshold {
            audio.queue(AudioPlay {
                path: format!("{}/pain", actor.kind.get_assets_path()).into(),
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
                    path: format!("{}/death", actor.kind.get_assets_path()).into(),
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

                blood += actor.radius * BLOOD_FACTOR_ON_DEATH;
                commands.entity(entity).despawn_recursive();
            }
        }

        let blood = BloodSpawn::new(point, blood);

        if !blood.is_too_small() {
            commands.add(blood);
        }

        health.commit();
    }
}
