use crate::{
    command::{ActorRelease, BloodSpawn},
    component::{Actor, Health},
    event::ActorDeathEvent,
    resource::AudioTracker,
};
use bevy::{
    ecs::system::{Query, ResMut},
    math::Vec3Swizzles,
    prelude::{Commands, DespawnRecursiveExt, Entity, EventWriter, Transform},
};

const BLOOD_MIN: f32 = 0.1;
const BLOOD_FACTOR_ON_DAMAGE: f32 = 24.0;
const BLOOD_FACTOR_ON_DEATH: f32 = 16.0;

pub fn health(
    mut query: Query<(Entity, &Actor, &mut Health, &Transform)>,
    mut death_events: EventWriter<ActorDeathEvent>,
    mut audio: ResMut<AudioTracker>,
    mut commands: Commands,
) {
    for (entity, actor, mut health, transform) in query.iter_mut() {
        let actor = actor.config;
        let point = transform.translation.xy();
        let damage = health.get_damage();
        let mut blood = actor.radius * damage * BLOOD_FACTOR_ON_DAMAGE;

        if health.is_alive() && damage > actor.pain_threshold {
            if let Some(sound) = &actor.sound_pain {
                audio.queue(sound.as_spatial(point));
            }
        }

        if health.is_just_died() {
            if let Some(sound) = &actor.sound_death {
                audio.queue(sound.as_spatial(point));
            }

            commands.add(ActorRelease(entity));
            death_events.send(ActorDeathEvent::new(point));
            blood += actor.radius * BLOOD_FACTOR_ON_DEATH;
            commands.entity(entity).despawn_recursive();
        }

        if blood > BLOOD_MIN {
            commands.add(BloodSpawn::new(point, blood));
        }

        health.commit();
    }
}
