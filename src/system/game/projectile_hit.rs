use crate::{
    command::AudioPlay,
    component::{Health, Inertia},
};
use bevy::{
    ecs::{entity::Entity, system::Query},
    math::Vec3Swizzles,
    prelude::{Commands, In, Transform, Vec2},
};

pub fn projectile_hit(
    In(mut hits): In<Vec<(Entity, Vec2, f32)>>,
    mut entities: Query<(&Transform, &mut Inertia, &mut Health)>,
    mut commands: Commands,
) {
    while let Some((entity, force, force_angular)) = hits.pop() {
        let momentum = force.length();

        if let Ok((transform, mut inertia, mut health)) = entities.get_mut(entity) {
            inertia.push(force, momentum * force_angular, true, false, true);
            health.damage(momentum);

            // TODO: don't play multiple times if it was a fraction
            commands.add(AudioPlay {
                path: "sounds/hit_body_{n}.ogg",
                volume: 1.5,
                source: Some(transform.translation.xy()),
                ..AudioPlay::DEFAULT
            });
        }
    }
}
