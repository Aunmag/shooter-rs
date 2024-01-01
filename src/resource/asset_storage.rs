use bevy::{
    asset::{AssetServer, Handle, LoadedFolder},
    ecs::system::Resource,
};

#[derive(Default, Resource)]
pub struct AssetStorage {
    handle: Option<Handle<LoadedFolder>>,
}

impl AssetStorage {
    pub fn load(&mut self, asset_server: &AssetServer) {
        self.handle = Some(asset_server.load_folder("."));
    }

    pub fn is_loaded(&self, asset_server: &AssetServer) -> Option<bool> {
        return self
            .handle
            .as_ref()
            .map(|h| asset_server.is_loaded_with_dependencies(h.id()));
    }
}
