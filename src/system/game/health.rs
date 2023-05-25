use crate::command::EntityDelete;
use crate::component::Actor;
use crate::component::Health;
use crate::resource::Rng;
use bevy::ecs::system::Query;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Commands;
use bevy::prelude::Entity;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Time;
use bevy::prelude::Transform;

pub fn health(
    mut query: Query<(Entity, &Actor, &mut Health, &Transform)>,
    mut rng: ResMut<Rng>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let now = time.elapsed();

    for (entity, actor, mut health, transform) in query.iter_mut() {
        let change = health.change();

        if health.is_alive() {
            if -change > actor.config.pain_threshold {
                if let Some(sound) = &actor.config.sound_pain {
                    sound.maybe_play(transform.translation.xy(), &mut rng, &mut commands);
                }
            }
        } else if change != 0.0 {
            if let Some(sound) = &actor.config.sound_death {
                sound.maybe_play(transform.translation.xy(), &mut rng, &mut commands);
            }
        }

        if health.is_decayed(now) {
            commands.add(EntityDelete(entity));
        }

        health.save_change();
    }
}
