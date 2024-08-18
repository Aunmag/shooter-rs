use crate::{
    data::{LAYER_CROSSHAIR, PIXELS_PER_METER},
    model::AppState,
    plugin::{camera::MainCamera, player::Player},
    resource::AssetStorage,
    util::{
        ext::{AppExt, Vec2Ext},
        math::angle_difference,
    },
};
use bevy::{
    app::{App, Plugin},
    asset::{Asset, Assets, Handle},
    ecs::{
        entity::Entity,
        query::{With, Without},
        schedule::IntoSystemConfigs,
        system::Query,
        world::World,
    },
    input::mouse::MouseMotion,
    math::{Vec2, Vec3},
    prelude::{EventReader, Image, Transform},
    reflect::TypePath,
    render::{
        camera::{Camera, OrthographicProjection},
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    transform::components::GlobalTransform,
};

const SIZE: f32 = PIXELS_PER_METER * 1.2;

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<Crosshair>::default());
        app.add_state_system(
            AppState::Game,
            on_update.after(crate::plugin::camera_target::on_update),
        );
    }
}

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
pub struct Crosshair {
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
}

impl Crosshair {
    pub fn spawn(world: &mut World) -> Entity {
        let assets = world.resource::<AssetStorage>();
        let image = assets.dummy_image().clone();
        let mesh = assets.dummy_mesh().clone();
        let material = world
            .resource_mut::<Assets<Crosshair>>()
            .add(Crosshair { image });

        return world
            .spawn(MaterialMesh2dBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, LAYER_CROSSHAIR),
                    ..Transform::default()
                },
                mesh: mesh.into(),
                material,
                ..Default::default()
            })
            .id();
    }
}

impl Material2d for Crosshair {
    fn fragment_shader() -> ShaderRef {
        return "shader/crosshair.wgsl".into();
    }
}

#[allow(clippy::unwrap_used)]
fn on_update(
    mut crosshairs: Query<&mut Transform, (With<Handle<Crosshair>>, Without<Player>)>,
    cameras: Query<(&Camera, &GlobalTransform, &OrthographicProjection), With<MainCamera>>,
    mut players: Query<(&mut Player, &mut Transform)>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let mut cursor_delta = Vec2::ZERO;

    for event in mouse_motion.read() {
        cursor_delta += event.delta;
    }

    let Some((camera, camera_transform, camera_projection)) = cameras.iter().next() else {
        return;
    };

    for (mut player, mut player_transform) in players.iter_mut() {
        let Some(crosshair) = player.crosshair.as_mut() else {
            continue;
        };

        let Ok(mut transform) = crosshairs.get_mut(crosshair.entity) else {
            continue;
        };

        let player_position = player_transform.translation.truncate();

        // crosshair must in sync with player while it moves, also player direction can be
        // changed because of weapon recoil, so crosshair shod be affected too
        let on_world = player_position
            + Vec2::new(crosshair.distance, 0.0).rotate_by_quat(player_transform.rotation);

        let Some(on_screen_old) = camera.world_to_viewport(camera_transform, on_world.extend(0.0))
        else {
            continue;
        };

        let mut on_screen_new = on_screen_old + cursor_delta;

        // clamp crosshair inside view port
        if let Some(viewport_size) = camera.logical_viewport_size() {
            on_screen_new.x = on_screen_new.x.clamp(0.0, viewport_size.x);
            on_screen_new.y = on_screen_new.y.clamp(0.0, viewport_size.y);
        }

        transform.scale.x = SIZE * camera_projection.scale;
        transform.scale.y = SIZE * camera_projection.scale;

        // put crosshair to it's updated position
        let on_world_new = camera
            .viewport_to_world(camera_transform, on_screen_new)
            .unwrap()
            .origin
            .truncate();

        transform.translation.x = on_world_new.x;
        transform.translation.y = on_world_new.y;

        if on_screen_new != on_screen_old {
            // update crosshair distance only when it'd moved. otherwise distance error may grow
            crosshair.distance = player_position.distance(on_world_new);

            // if crosshair had rotated, i.e. by input, then rote the player too
            player_transform.rotate_local_z(angle_difference(
                player_position.angle_to(on_world),
                player_position.angle_to(on_world_new),
            ));
        }

        transform.rotation = player_transform.rotation;
    }
}
