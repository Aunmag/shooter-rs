use crate::{
    component::{Health, Inertia, Player},
    resource::HitResource,
};
use bevy::prelude::{Query, ResMut};

pub fn hit(
    mut targets: Query<(&mut Inertia, &mut Health, Option<&mut Player>)>,
    mut hits: ResMut<HitResource>,
) {
    for hit in hits.hits.drain(..) {
        if let Ok((mut inertia, mut health, mut player)) = targets.get_mut(hit.entity) {
            let momentum = hit.force.length();
            let force_angular = hit.angle * momentum;
            inertia.push(hit.force, force_angular, true, false, true);
            health.damage(momentum);

            if let Some(player) = player.as_mut() {
                player.shake(force_angular * Inertia::PUSH_MULTIPLIER_ANGULAR);
            }
        }
    }
}
