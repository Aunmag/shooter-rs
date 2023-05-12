use crate::component::Health;
use crate::component::Inertia;
use crate::component::Projectile;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Query;
use bevy::prelude::In;
use bevy::prelude::Res;
use bevy::prelude::Time;
use bevy::prelude::Vec2;

pub fn projectile_hit(
    In(mut hits): In<Vec<(Entity, Vec2, f32)>>,
    mut entities: Query<(&mut Inertia, &mut Health)>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    while let Some((entity, force, force_angular)) = hits.pop() {
        let momentum = force.length();

        if let Ok((mut inertia, mut health)) = entities.get_mut(entity) {
            inertia.push(
                force * Projectile::PUSH_FACTOR,
                momentum * force_angular * Projectile::PUSH_FACTOR_ANGULAR,
                true,
                false,
            );
            health.damage(momentum, time);
        }
    }
}
