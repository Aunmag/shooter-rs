use bevy::{
    prelude::{Handle, Image},
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "5a5b4fc1-2055-4ac5-ade7-ecf1e3aa2c1b"]
pub struct LaserMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub image: Handle<Image>,
}

impl Material2d for LaserMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shader/laser.wgsl".into();
    }
}
