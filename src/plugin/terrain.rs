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
use rand::seq::SliceRandom;

const PATH: &str = "terrain/grass_513473301.png";
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
    let images = [
        "terrain/dead_3174437271.png",
        "terrain/dead_330027463.png",
        "terrain/dead_980120234.png",
        "terrain/desert_0.png",
        "terrain/desert_1.png",
        "terrain/desert_1552516878.png",
        "terrain/desert_1552516880.png",
        "terrain/desert_1552516885.png",
        "terrain/desert_1552516897.png",
        "terrain/desert_1552516899.png",
        "terrain/desert_4243541191.png",
        "terrain/desert_4243541200.png",
        "terrain/desert_4243541201.png",
        "terrain/dirt_moss_pebble_330027463.png",
        "terrain/dirt_moss_pebble_980120234.png",
        "terrain/dry_2952353678.png",
        "terrain/dry_3792346351.png",
        "terrain/grass_3174437271.png",
        "terrain/grass_513473301.png",
        "terrain/grass_533163216.png",
        "terrain/grass_533163253.png",
        "terrain/hell_3849968848.png",
        "terrain/hell_3849968862.png",
        "terrain/hell_992658657.png",
    ];

    let index = if keyboard.just_pressed(KeyCode::Key0) {
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
    } else if keyboard.just_pressed(KeyCode::Key7) {
        7
    } else if keyboard.just_pressed(KeyCode::Key8) {
        8
    } else if keyboard.just_pressed(KeyCode::Key9) {
        9
    } else if keyboard.just_pressed(KeyCode::Equals) {
        usize::MAX
    } else {
        return;
    };

    let image = if index == usize::MAX {
        let mut rng = rand::thread_rng();
        images.choose(&mut rng)
    } else {
        images.get(index)
    };

    let Some(&image) = image else {
        return;
    };

    commands.add(move |w: &mut World| {
        let Some(image_handle) = w.resource::<AssetServer>().get_handle::<Image>(image) else {
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
