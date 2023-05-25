use crate::command::EntityDelete;
use crate::component::Actor;
use crate::component::Health;
use bevy::ecs::system::Query;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Commands;
use bevy::prelude::Entity;
use bevy::prelude::Res;
use bevy::prelude::Time;
use bevy::prelude::Transform;

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
            commands.add(EntityDelete(entity));
        }

        health.save_change();
    }
}
