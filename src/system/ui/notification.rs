use crate::component::Notification;
use bevy::{
    prelude::{Commands, DespawnRecursiveExt, Entity, Query, Res},
    text::Text,
    time::Time,
};

pub fn notification(
    mut query: Query<(Entity, &Notification, &mut Text)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (entity, notification, mut text) in query.iter_mut() {
        if notification.is_expired(time) {
            commands.entity(entity).despawn_recursive();
        } else {
            let alpha = notification.alpha(time);

            for section in text.sections.iter_mut() {
                section.style.color.set_a(alpha);
            }
        }
    }
}
