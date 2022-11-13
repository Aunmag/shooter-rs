use bevy::prelude::Handle;
use bevy::prelude::Image;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::sprite::Material2d;

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a741d840-3782-4b3b-8e86-7746c272ea63"]
pub struct ProjectileMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub image: Handle<Image>,
}

impl Material2d for ProjectileMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shader/projectile.wgsl".into();
    }
}
