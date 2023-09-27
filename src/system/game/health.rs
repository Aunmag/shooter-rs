use crate::{
    command::BloodSpawn,
    component::{Actor, Health, Voice, VoiceSound},
    event::ActorDeathEvent,
};
use bevy::{
    ecs::system::{Query, Res},
    math::Vec3Swizzles,
    prelude::{Commands, DespawnRecursiveExt, Entity, EventWriter, Transform},
    time::Time,
};

const BLOOD_MIN: f32 = 0.1;
const BLOOD_FACTOR_ON_DAMAGE: f32 = 24.0;
const BLOOD_FACTOR_ON_DEATH: f32 = 16.0;

pub fn health(
    mut query: Query<(Entity, &Actor, &mut Health, &Transform, &mut Voice)>,
    mut death_events: EventWriter<ActorDeathEvent>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let now = time.elapsed();

    for (entity, actor, mut health, transform, mut voice) in query.iter_mut() {
        let actor = actor.config;
        let point = transform.translation.xy();
        let damage = health.get_damage();
        let mut blood = actor.radius * damage * BLOOD_FACTOR_ON_DAMAGE;

        if health.is_alive() && damage > actor.pain_threshold {
            voice.queue(VoiceSound::Pain, now);
        }

        if !health.is_alive() {
            if health.is_just_died() {
                blood += actor.radius * BLOOD_FACTOR_ON_DEATH;
                death_events.send(ActorDeathEvent::new(
                    entity,
                    actor.kind,
                    point,
                    health.get_attacker(),
                ));
            } else {
                // later despawning since we need more time to play queued death sound
                commands.entity(entity).despawn_recursive();
            }
        }

        if blood > BLOOD_MIN {
            commands.add(BloodSpawn::new(point, blood));
        }

        health.commit();
    }
}
