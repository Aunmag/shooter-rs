use crate::component::Inertia;
use bevy::math::Quat;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::Time;
use bevy::prelude::Transform;

pub fn inertia(mut query: Query<(&mut Transform, &mut Inertia)>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (mut transform, mut inertia) in query.iter_mut() {
        transform.translation.x += inertia.velocity.x * delta;
        transform.translation.y += inertia.velocity.y * delta;
        transform.rotate(Quat::from_rotation_z(inertia.spinning * delta));
        inertia.velocity *= 1.0 - delta * Inertia::DRAG;
        inertia.spinning *= 1.0 - delta * Inertia::DRAG_ANGULAR;
    }
}
