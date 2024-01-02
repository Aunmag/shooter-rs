use crate::{
    component::{Health, Inertia, Player},
    resource::HitResource,
};
use bevy::{
    ecs::{
        schedule::{IntoSystemConfigs, SystemConfigs},
        system::Res,
    },
    prelude::{Query, ResMut},
};

pub fn hit_inner(
    mut targets: Query<(&mut Inertia, &mut Health, Option<&mut Player>)>,
    mut hits: ResMut<HitResource>,
) {
    for hit in hits.hits.drain(..) {
        if let Ok((mut inertia, mut health, mut player)) = targets.get_mut(hit.entity) {
            let momentum_linear = hit.momentum.length();
            let momentum_angular = momentum_linear * hit.angle;

            inertia.push(hit.momentum, momentum_angular, false, true);
            health.damage(momentum_linear);

            if let Some(player) = player.as_mut() {
                player.shake(momentum_angular * Inertia::PUSH_MULTIPLIER_ANGULAR);
            }
        }
    }
}

pub fn hit() -> SystemConfigs {
    return hit_inner.run_if(|r: Res<HitResource>| !r.hits.is_empty());
}
