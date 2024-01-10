use bevy::{
    asset::Asset,
    prelude::{Handle, Image},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
pub struct CrosshairMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub image: Handle<Image>,
}

impl Material2d for CrosshairMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shader/crosshair.wgsl".into();
    }
}
