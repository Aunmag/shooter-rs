use crate::{
    command::AudioPlay,
    component::{Health, Inertia, Player},
};
use bevy::{
    ecs::{entity::Entity, system::Query},
    math::Vec3Swizzles,
    prelude::{Commands, In, Transform, Vec2},
};

pub fn projectile_hit(
    In(mut hits): In<Vec<(Entity, Vec2, f32)>>,
    mut entities: Query<(&Transform, &mut Inertia, &mut Health, Option<&mut Player>)>,
    mut commands: Commands,
) {
    let mut unique_hits = Vec::with_capacity(hits.capacity());

    while let Some((entity, force, force_angular)) = hits.pop() {
        let momentum = force.length();

        if let Ok((transform, mut inertia, mut health, mut player)) = entities.get_mut(entity) {
            let force_angular = force_angular * momentum;
            inertia.push(force, force_angular, true, false, true);
            health.damage(momentum);

            if let Some(player) = player.as_mut() {
                player.shake(force_angular * Inertia::PUSH_MULTIPLIER_ANGULAR);
            }

            if !unique_hits.contains(&entity.index()) {
                unique_hits.push(entity.index());
                commands.add(AudioPlay {
                    path: "sounds/hit_body_{n}.ogg",
                    volume: 1.5,
                    source: Some(transform.translation.xy()),
                    priority: AudioPlay::PRIORITY_LOWER,
                    ..AudioPlay::DEFAULT
                });
            }
        }
    }
}
