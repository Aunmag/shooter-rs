use bevy::{
    asset::Asset,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d},
};

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
pub struct ProjectileMaterial {}

impl Material2d for ProjectileMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shader/projectile.wgsl".into();
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }
}
