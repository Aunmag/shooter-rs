use bevy::{
    asset::Asset,
    prelude::{Handle, Image},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use std::time::Duration;

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
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
