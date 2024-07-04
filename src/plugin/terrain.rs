use crate::{
    data::{LAYER_BACKGROUND, PIXELS_PER_METER, TRANSFORM_SCALE, VIEW_DISTANCE},
    model::AppState,
    plugin::sys_camera_target,
    util::{
        ext::{AppExt, ImageExt},
        math::round_by,
    },
};
use bevy::{
    app::{App, Plugin}, asset::{AssetServer, Assets, Handle}, ecs::{component::Component, schedule::IntoSystemConfigs, system::Query, world::World}, input::Input, math::Vec3, prelude::{Camera, Commands, Entity, KeyCode, Res, Transform, With, Without}, render::{
        mesh::{shape::Quad, Mesh, VertexAttributeValues},
        texture::{Image, ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
    }, sprite::{ColorMaterial, ColorMesh2dBundle}
};

const PATH: &str = "terrain/1.png";
const DEBUG: bool = true; // TODO: disable by default

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system_enter(AppState::Game, on_init);
        app.add_state_system(AppState::Game, on_update.after(sys_camera_target));

        if DEBUG {
            app.add_state_system(AppState::Game, on_debug_update);
        }
    }
}

#[derive(Component)]
struct Terrain {
    block_size: f32,
}

fn on_init(world: &mut World) {
    let Some(image_handle) = world.resource::<AssetServer>().get_handle(PATH).clone() else {
        log::warn!("Image {} not found", PATH);
        return;
    };

    let block_size;

    if let Some(image) = world.resource_mut::<Assets<Image>>().get_mut(&image_handle) {
        image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            ..Default::default()
        });

        block_size = u32::min(image.size_x(), image.size_y()) as f32 / PIXELS_PER_METER;
    } else {
        return;
    }

    let count = calc_count(block_size);

    let mut mesh = Mesh::from(Quad::default());
    if let Some(VertexAttributeValues::Float32x2(uvs)) = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        for uv in uvs {
            uv[0] *= count;
            uv[1] *= count;
        }
    }

    let mesh_handle = world.resource_mut::<Assets<Mesh>>().add(mesh);

    let material_handle = world
        .resource_mut::<Assets<ColorMaterial>>()
        .add(image_handle.into());

    world
        .spawn(ColorMesh2dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, LAYER_BACKGROUND),
                scale: TRANSFORM_SCALE * count * block_size * PIXELS_PER_METER,
                ..Default::default()
            },
            mesh: mesh_handle.into(),
            material: material_handle,
            ..Default::default()
        })
        .insert(Terrain { block_size });
}

fn on_update(
    mut terrains: Query<(&Terrain, &mut Transform), Without<Camera>>,
    cameras: Query<&Transform, With<Camera>>,
) {
    let Some(center) = cameras.iter().next().map(|c| c.translation.truncate()) else {
        return;
    };

    for (terrain, mut transform) in terrains.iter_mut() {
        transform.translation.x = round_by(center.x, terrain.block_size);
        transform.translation.y = round_by(center.y, terrain.block_size);
    }
}

fn on_debug_update(
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    let id = if keyboard.just_pressed(KeyCode::Key0) {
        0
    } else if keyboard.just_pressed(KeyCode::Key1) {
        1
    } else if keyboard.just_pressed(KeyCode::Key2) {
        2
    } else if keyboard.just_pressed(KeyCode::Key3) {
        3
    } else if keyboard.just_pressed(KeyCode::Key4) {
        4
    } else if keyboard.just_pressed(KeyCode::Key5) {
        5
    } else if keyboard.just_pressed(KeyCode::Key6) {
        6
    } else {
        return;
    };

    let image = format!("terrain/{}.png", id);

    commands.add(move |w: &mut World| {
        let Some(image_handle) = w.resource::<AssetServer>().get_handle::<Image>(&image) else {
            return;
        };

        if let Some(image) = w.resource_mut::<Assets<Image>>().get_mut(&image_handle) {
            image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..Default::default()
            });
        } else {
            return;
        }

        let material = w.resource_mut::<Assets<ColorMaterial>>().add(image_handle.into());

        for entity in w.query_filtered::<Entity, With<Terrain>>().iter(&w).collect::<Vec<_>>() {
            w.entity_mut(entity)
                .insert(material.clone())
                ;
        }
    });
}

fn calc_count(block_size: f32) -> f32 {
    return (VIEW_DISTANCE / block_size + 2.0).ceil();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_count() {
        assert_eq!(calc_count(4.0), 12.0);
    }
}
