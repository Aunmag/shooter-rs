use crate::component::{Actor, Health};
use bevy::{
    ecs::system::Query,
    math::Vec3Swizzles,
    prelude::{Commands, DespawnRecursiveExt, Entity, Res, Time, Transform},
};

pub fn health(
    mut query: Query<(Entity, &Actor, &mut Health, &Transform)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let now = time.elapsed();

    for (entity, actor, mut health, transform) in query.iter_mut() {
        let change = health.change();

        if health.is_alive() {
            if -change > actor.config.pain_threshold {
                if let Some(sound) = &actor.config.sound_pain {
                    commands.add(sound.as_spatial(transform.translation.xy()));
                }
            }
        } else if change != 0.0 {
            if let Some(sound) = &actor.config.sound_death {
                commands.add(sound.as_spatial(transform.translation.xy()));
            }
        }

        if health.is_decayed(now) {
            commands.entity(entity).despawn_recursive();
        }

        health.save_change();
    }
}
