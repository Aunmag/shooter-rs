use crate::util::ext::ImageExt;
use bevy::{
    asset::{AssetServer, Assets, Handle, LoadedFolder},
    ecs::system::Resource,
    render::{
        mesh::{shape::Quad, Mesh},
        texture::Image,
    },
};

#[derive(Default, Resource)]
pub struct AssetStorage {
    assets: Option<Handle<LoadedFolder>>,
    dummy_image: Handle<Image>,
    dummy_mesh: Handle<Mesh>,
}

impl AssetStorage {
    pub fn load(
        &mut self,
        asset_server: &AssetServer,
        images: &mut Assets<Image>,
        meshes: &mut Assets<Mesh>,
    ) {
        self.assets = Some(asset_server.load_folder("."));
        self.dummy_image = images.add(Image::blank(1, 1));
        self.dummy_mesh = meshes.add(Mesh::from(Quad::default()));
    }

    pub fn is_lading_started(&self) -> bool {
        return self.assets.is_some();
    }

    pub fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        return self
            .assets
            .as_ref()
            .map(|h| asset_server.is_loaded_with_dependencies(h.id()))
            .unwrap_or(false);
    }

    pub fn dummy_image(&self) -> &Handle<Image> {
        return &self.dummy_image;
    }

    pub fn dummy_mesh(&self) -> &Handle<Mesh> {
        return &self.dummy_mesh;
    }
}
