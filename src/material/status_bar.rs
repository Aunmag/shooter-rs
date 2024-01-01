use bevy::{
    asset::Asset,
    prelude::{Handle, Image},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
pub struct StatusBarMaterial {
    #[uniform(0)]
    pub health: f32,
    #[uniform(0)]
    pub health_alpha: f32,
    #[uniform(0)]
    pub ammo: f32,
    #[uniform(0)]
    pub ammo_alpha: f32,
    #[uniform(0)]
    pub stamina: f32,
    #[texture(1)]
    #[sampler(2)]
    pub image: Handle<Image>,
}

impl Material2d for StatusBarMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shader/status_bar.wgsl".into();
    }
}
