use crate::{data::PIXELS_PER_METER, resource::AssetStorage};
use bevy::{
    app::{App, Plugin},
    asset::{Asset, Assets},
    prelude::{BuildWorldChildren, Entity, Handle, Image, Quat, Transform, Vec3, World},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};
use std::f32::consts::PI;

const LENGTH: f32 = 26.0 * PIXELS_PER_METER;
const THICKNESS: f32 = 0.5 * PIXELS_PER_METER;

pub struct LaserSightPlugin;

impl Plugin for LaserSightPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<LaserSight>::default());
    }
}

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
pub struct LaserSight {
    #[texture(0)]
    #[sampler(1)]
    image: Handle<Image>,
}

impl LaserSight {
    pub fn spawn(world: &mut World, parent: Entity) {
        let assets = world.resource::<AssetStorage>();
        let image = assets.dummy_image().clone();
        let mesh = assets.dummy_mesh().clone();
        let material = world
            .resource_mut::<Assets<LaserSight>>()
            .add(LaserSight { image });

        world
            .spawn(MaterialMesh2dBundle {
                transform: Transform {
                    translation: Vec3::new(LENGTH / 2.0 + PIXELS_PER_METER / 2.0, 0.0, -1.0),
                    scale: Vec3::new(LENGTH, THICKNESS, 1.0),
                    rotation: Quat::from_rotation_z(PI),
                },
                mesh: mesh.into(),
                material,
                ..Default::default()
            })
            .set_parent(parent);
    }
}

impl Material2d for LaserSight {
    fn fragment_shader() -> ShaderRef {
        return "shader/laser.wgsl".into();
    }
}
