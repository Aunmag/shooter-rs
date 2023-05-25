use bevy::{
    prelude::{Handle, Image},
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "71682a00-fabd-4639-a0a5-e5a984d01fa6"]
pub struct StatusBarMaterial {
    #[uniform(0)]
    pub health: f32,
    #[uniform(0)]
    pub health_alpha: f32,
    #[uniform(0)]
    pub ammo: f32,
    #[uniform(0)]
    pub ammo_alpha: f32,
    #[texture(1)]
    #[sampler(2)]
    pub image: Handle<Image>,
}

impl Material2d for StatusBarMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shader/status_bar.wgsl".into();
    }
}
