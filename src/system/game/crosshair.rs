use crate::{
    component::Player, data::PIXELS_PER_METER, material::CrosshairMaterial, util::ext::Vec2Ext,
};
use bevy::{
    asset::Handle,
    ecs::{
        query::{With, Without},
        system::Query,
    },
    input::mouse::MouseMotion,
    math::Vec2,
    prelude::{EventReader, Transform},
    render::camera::{Camera, OrthographicProjection},
    transform::components::GlobalTransform,
};

const SIZE: f32 = PIXELS_PER_METER * 1.2;

#[allow(clippy::unwrap_used)]
pub fn crosshair(
    mut crosshairs: Query<&mut Transform, (With<Handle<CrosshairMaterial>>, Without<Player>)>,
    cameras: Query<(&Camera, &GlobalTransform, &OrthographicProjection)>,
    mut players: Query<(&mut Player, &Transform)>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let mut cursor_delta = Vec2::ZERO;

    for event in mouse_motion.read() {
        cursor_delta += event.delta;
    }

    let Some((camera, camera_transform, camera_projection)) = cameras.iter().next() else {
        return;
    };

    for (mut player, player_transform) in players.iter_mut() {
        let player_position = player_transform.translation.truncate();

        let player_on_screen = camera
            .world_to_viewport(camera_transform, player_position.extend(0.0))
            .unwrap();

        if let Ok(mut transform) = crosshairs.get_mut(player.crosshair.entity) {
            // crosshair must in sync with player while it moves, also player direction can be
            // changed because of weapon recoil, so crosshair shod be affected too
            let on_world = player_position
                + Vec2::new(player.crosshair.distance, 0.0)
                    .rotate_by_quat(player_transform.rotation);

            let on_screen_old = camera
                .world_to_viewport(camera_transform, on_world.extend(0.0))
                .unwrap();

            let mut on_screen_new = on_screen_old;
            on_screen_new.y += cursor_delta.y;

            // clamp crosshair inside view port
            if let Some(viewport_size) = camera.logical_viewport_size() {
                on_screen_new.x = on_screen_new.x.clamp(0.0, viewport_size.x);
                on_screen_new.y = on_screen_new.y.clamp(0.0, viewport_size.y);
            }

            // don't allow crosshair go below player
            on_screen_new.y = f32::min(on_screen_new.y, player_on_screen.y);

            // put crosshair to it's updated position
            let on_world_new = camera
                .viewport_to_world(camera_transform, on_screen_new)
                .unwrap()
                .origin
                .truncate();

            transform.translation.x = on_world_new.x;
            transform.translation.y = on_world_new.y;
            transform.scale.x = SIZE * camera_projection.scale;
            transform.scale.y = SIZE * camera_projection.scale;

            if on_screen_new != on_screen_old {
                // update crosshair distance only when it'd moved. otherwise distance error may grow
                player.crosshair.distance = player_position.distance(on_world_new);
            }

            transform.rotation = player_transform.rotation;
        }
    }
}
