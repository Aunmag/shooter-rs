use crate::component::Inertia;
use bevy::prelude::{Query, Res, Time, Transform};

pub fn inertia(mut query: Query<(&mut Transform, &mut Inertia)>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (mut transform, mut inertia) in query.iter_mut() {
        transform.translation.x += inertia.velocity.x * delta;
        transform.translation.y += inertia.velocity.y * delta;
        transform.rotate_local_z(inertia.velocity_angular * delta);

        let drag = 1.0 - inertia.drag() * delta;
        inertia.velocity *= drag;
        inertia.velocity_angular *= drag;
    }
}
