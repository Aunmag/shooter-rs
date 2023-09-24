use bevy::{
    prelude::{Handle, Image},
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use std::time::Duration;

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "fe5e7c73-1d08-4cee-b2f7-25ab9b376c6a"]
pub struct BloodMaterial {
    pub spawned: Duration,
    #[uniform(0)]
    pub seed: f32,
    #[uniform(0)]
    pub size: f32,
    #[uniform(0)]
    pub spread: f32,
    #[texture(1)]
    #[sampler(2)]
    pub image: Handle<Image>,
}

impl Material2d for BloodMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shader/blood.wgsl".into();
    }
}
