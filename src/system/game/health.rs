use crate::{
    component::{Actor, Health},
    event::ActorDeathEvent,
};
use bevy::{
    ecs::system::Query,
    math::Vec3Swizzles,
    prelude::{Commands, DespawnRecursiveExt, Entity, EventWriter, Res, Time, Transform},
};
use std::time::Duration;

const DECAY: Duration = Duration::from_millis(500);

pub fn health(
    mut query: Query<(Entity, &Actor, &mut Health, &Transform)>,
    time: Res<Time>,
    mut death_events: EventWriter<ActorDeathEvent>,
    mut commands: Commands,
) {
    let now = time.elapsed();

    for (entity, actor, mut health, transform) in query.iter_mut() {
        let actor = actor.config;
        let point = transform.translation.xy();

        if health.is_alive() && health.get_damage() > actor.pain_threshold {
            if let Some(sound) = &actor.sound_pain {
                commands.add(sound.as_spatial(point));
            }
        }

        if health.is_just_died() {
            if let Some(sound) = &actor.sound_death {
                commands.add(sound.as_spatial(point));
            }

            health.decay(now + DECAY);
            death_events.send(ActorDeathEvent::new(point));
        }

        if health.is_decayed(now) {
            commands.entity(entity).despawn_recursive();
        }

        health.commit();
    }
}
