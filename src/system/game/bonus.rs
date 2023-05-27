use crate::{
    command::BonusActivate,
    component::{Actor, ActorType, Bonus, BonusImage, BonusLabel, Collision, Player},
    util::{ext::Vec2Ext, math::interpolate},
};
use bevy::{
    ecs::{entity::Entity, system::Res},
    math::Vec3Swizzles,
    prelude::{
        Commands, DespawnRecursiveExt, OrthographicProjection, Quat, Query, Vec3, With, Without,
    },
    time::Time,
    transform::components::Transform,
};
use std::f32::consts::TAU;

pub fn bonus(
    bonuses: Query<(Entity, &Bonus, &Transform)>,
    players: Query<(Entity, &Actor, &Transform, &Collision), With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let now = time.elapsed();

    for (bonus_entity, bonus, bonus_transform) in bonuses.iter() {
        if bonus.is_expired(now) {
            commands.entity(bonus_entity).despawn_recursive();
            continue;
        }

        let bonus_position = bonus_transform.translation.xy();

        for (player_entity, actor, player_transform, player_body) in players.iter() {
            if actor.config.actor_type != ActorType::Human {
                continue;
            }

            let player_position = player_transform.translation.xy();

            if (bonus_position - player_position)
                .is_shorter_than(Bonus::RADIUS + player_body.radius)
            {
                commands.add(BonusActivate::new(bonus_entity, player_entity));
                break;
            }
        }
    }
}

pub fn bonus_image(mut query: Query<&mut Transform, With<BonusImage>>, time: Res<Time>) {
    let rotation = time.elapsed_seconds() % Bonus::PULSE.as_secs_f32() * -TAU;
    for mut image in query.iter_mut() {
        image.rotation = Quat::from_rotation_z(rotation);
    }
}

pub fn bonus_label(
    mut query: Query<&mut Transform, (With<BonusLabel>,)>,
    cameras: Query<&Transform, (With<OrthographicProjection>, Without<BonusLabel>)>,
    time: Res<Time>,
) {
    let scale = interpolate(
        Bonus::TEXT_SCALE_MIN,
        Bonus::TEXT_SCALE_MAX,
        (time.elapsed_seconds() / Bonus::PULSE.as_secs_f32() * TAU * 2.0).cos(),
    );

    let rotation = cameras
        .iter()
        .next()
        .map_or_else(Default::default, |c| c.rotation);

    for mut label in query.iter_mut() {
        label.translation = rotation * Vec3::new(0.0, -5.0, 1.5);
        label.rotation = rotation;
        label.scale = Vec3::splat(scale);
    }
}
