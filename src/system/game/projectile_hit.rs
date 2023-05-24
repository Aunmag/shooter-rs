use crate::command::AudioPlay;
use crate::component::Health;
use crate::component::Inertia;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Query;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Commands;
use bevy::prelude::In;
use bevy::prelude::Res;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use bevy::prelude::Vec2;

pub fn projectile_hit(
    In(mut hits): In<Vec<(Entity, Vec2, f32)>>,
    mut entities: Query<(&Transform, &mut Inertia, &mut Health)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let time = time.elapsed();

    while let Some((entity, force, force_angular)) = hits.pop() {
        let momentum = force.length();

        if let Ok((transform, mut inertia, mut health)) = entities.get_mut(entity) {
            inertia.push(force, momentum * force_angular, true, false, true);

            health.damage(momentum, time);

            // TODO: don't play multiple times if it was a fraction
            commands.add(AudioPlay {
                path: "sounds/hit_body_{n}.ogg",
                volume: 1.5,
                source: Some(transform.translation.xy()),
                choices: 5,
                ..Default::default()
            });
        }
    }
}
