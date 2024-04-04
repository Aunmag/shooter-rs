use super::TileBlend;
use crate::{
    data::{LAYER_GROUND, PIXELS_PER_METER},
    resource::AssetStorage,
    util::math::interpolate_unbounded,
};
use bevy::{
    app::{App, Plugin},
    asset::Asset,
    ecs::system::Command,
    prelude::{Assets, Handle, Image, Transform, Vec2, Vec3, World},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};
use rand::{thread_rng, Rng};

const SIZE_MIN: f32 = 0.8;
const SIZE_MAX: f32 = 6.0;

pub struct BloodPlugin;

impl Plugin for BloodPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<Blood>::default());
    }
}

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
pub struct Blood {
    #[uniform(0)]
    seed: f32,
    #[uniform(0)]
    size: f32,
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
}

impl Material2d for Blood {
    fn fragment_shader() -> ShaderRef {
        return "shader/blood.wgsl".into();
    }
}

pub struct BloodSpawn {
    position: Vec2,
    size: f32,
}

impl BloodSpawn {
    pub fn new(position: Vec2, scale: f32) -> Option<Self> {
        let size = interpolate_unbounded(0.0, SIZE_MAX, f32::min(scale, 1.0));

        if size < SIZE_MIN {
            return None;
        } else {
            return Some(Self { position, size });
        }
    }
}

impl Command for BloodSpawn {
    fn apply(self, world: &mut World) {
        let assets = world.resource::<AssetStorage>();
        let image = assets.dummy_image().clone();
        let mesh = assets.dummy_mesh().clone();

        let material = world.resource_mut::<Assets<Blood>>().add(Blood {
            seed: thread_rng().gen_range(0.0..500.0),
            size: self.size * PIXELS_PER_METER,
            image,
        });

        let entity = world
            .spawn(MaterialMesh2dBundle {
                transform: Transform {
                    translation: self.position.extend(LAYER_GROUND),
                    scale: Vec3::splat(self.size),
                    ..Transform::default()
                },
                mesh: mesh.into(),
                material,
                ..Default::default()
            })
            .id();

        TileBlend::Entity(entity).apply(world);
    }
}
