use bevy::{
    prelude::{Color, Handle, Image},
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "71682a00-fabd-4639-a0a5-e5a984d01fa6"]
pub struct HealthBarMaterial {
    #[uniform(0)]
    pub value: f32,
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub thickness: f32,
    #[texture(1)]
    #[sampler(2)]
    pub image: Handle<Image>,
}

impl Material2d for HealthBarMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shader/health_bar.wgsl".into();
    }
}
