use bevy::{
    asset::Handle,
    ecs::system::Resource,
    prelude::{Image, Mesh},
};

#[derive(Default, Resource)]
pub struct Cache {
    pub dummy_image: Option<Handle<Image>>,
    pub dummy_mesh: Option<Handle<Mesh>>,
}
