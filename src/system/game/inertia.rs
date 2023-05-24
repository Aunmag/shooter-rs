use crate::component::Inertia;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::Time;
use bevy::prelude::Transform;

pub fn inertia(mut query: Query<(&mut Transform, &mut Inertia)>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (mut transform, mut inertia) in query.iter_mut() {
        transform.translation.x += inertia.velocity.x * delta;
        transform.translation.y += inertia.velocity.y * delta;
        transform.rotate_local_z(inertia.velocity_angular * delta);
        inertia.velocity *= 1.0 - delta * Inertia::DRAG;
        inertia.velocity_angular *= 1.0 - delta * Inertia::DRAG_ANGULAR;
    }
}
