use crate::{
    data::{LAYER_CROSSHAIR, PIXELS_PER_METER},
    plugin::{camera::MainCamera, player::Player},
    resource::AssetStorage,
    state::AppState,
    util::ext::{AppExt, QuatExt, Vec2Ext},
};
use bevy::{
    app::{App, Plugin},
    asset::{Asset, Assets, Handle},
    ecs::{
        entity::Entity,
        query::{With, Without},
        schedule::IntoScheduleConfigs,
        system::Query,
        world::World,
    },
    input::mouse::MouseMotion,
    math::{Vec2, Vec3},
    prelude::{EventReader, Image, Transform},
    reflect::TypePath,
    render::{
        camera::{Camera, Projection},
        mesh::Mesh2d,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::{AlphaMode2d, Material2d, Material2dPlugin, MeshMaterial2d},
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
            .spawn((
                Transform {
                    translation: Vec3::new(0.0, 0.0, LAYER_CROSSHAIR),
                    ..Transform::default()
                },
                Mesh2d(mesh),
                MeshMaterial2d(material),
            ))
            .id();
    }
}

impl Material2d for Crosshair {
    fn fragment_shader() -> ShaderRef {
        return "shader/crosshair.wgsl".into();
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }
}

fn on_update(
    mut crosshairs: Query<&mut Transform, (With<MeshMaterial2d<Crosshair>>, Without<Player>)>,
    cameras: Query<(&Camera, &GlobalTransform, &Projection), With<MainCamera>>,
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

        // crosshair must in sync with player while it moves, also player direction can be changed
        // because of weapon recoil, so crosshair should be affected too
        let on_world_old =
            player_position + player_transform.rotation.as_vec() * crosshair.distance;

        let Ok(on_screen_old) =
            camera.world_to_viewport(camera_transform, on_world_old.extend(0.0))
        else {
            continue;
        };

        let mut on_screen_new = on_screen_old + cursor_delta;

        // clamp crosshair inside view port
        if let Some(viewport_size) = camera.logical_viewport_size() {
            on_screen_new.x = on_screen_new.x.clamp(0.0, viewport_size.x);
            on_screen_new.y = on_screen_new.y.clamp(0.0, viewport_size.y);
        }

        if let Projection::Orthographic(projection) = camera_projection {
            transform.scale.x = SIZE * projection.scale;
            transform.scale.y = SIZE * projection.scale;
        }

        // put crosshair to it's updated position
        if let Ok(on_world_new) = camera
            .viewport_to_world(camera_transform, on_screen_new)
            .map(|v| v.origin.truncate())
        {
            transform.translation.x = on_world_new.x;
            transform.translation.y = on_world_new.y;

            // update only when cursor moved more than 1px actually, otherwise errors may grow
            if (on_screen_new - on_screen_old).is_long(1.0) {
                crosshair.distance = player_position.distance(on_world_new);
                player_transform.rotation = (on_world_new - player_position).as_quat();
            }

            transform.rotation = player_transform.rotation;
        }
    }
}
