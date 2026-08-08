use bevy::{
    asset::{AssetServer, Assets, Handle, LoadedFolder},
    ecs::resource::Resource,
    mesh::Mesh,
    prelude::Rectangle,
};

#[derive(Default, Resource)]
pub struct AssetStorage {
    assets: Option<Handle<LoadedFolder>>,
    dummy_mesh: Handle<Mesh>,
}

impl AssetStorage {
    pub fn load(&mut self, asset_server: &AssetServer, meshes: &mut Assets<Mesh>) {
        self.assets = Some(asset_server.load_folder("."));
        self.dummy_mesh = meshes.add(Mesh::from(Rectangle::default()));
    }

    pub fn is_lading_started(&self) -> bool {
        return self.assets.is_some();
    }

    pub fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        return self
            .assets
            .as_ref()
            .is_some_and(|h| asset_server.is_loaded_with_dependencies(h.id()));
    }

    pub fn dummy_mesh(&self) -> &Handle<Mesh> {
        return &self.dummy_mesh;
    }
}
