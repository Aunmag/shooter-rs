use crate::resource::AssetStorage;
use bevy::prelude::{AssetServer, Res, ResMut};

pub fn on_enter(asset_server: Res<AssetServer>, mut asset_storage: ResMut<AssetStorage>) {
    load_folder_or_log(&asset_server, &mut asset_storage, "actors");
    load_folder_or_log(&asset_server, &mut asset_storage, "fonts");
    load_folder_or_log(&asset_server, &mut asset_storage, "sounds");
    load_folder_or_log(&asset_server, &mut asset_storage, "terrain");
    load_folder_or_log(&asset_server, &mut asset_storage, "weapons");
}

fn load_folder_or_log(
    asset_server: &Res<AssetServer>,
    asset_storage: &mut ResMut<AssetStorage>,
    path: &str,
) {
    match asset_server.load_folder(path) {
        Ok(assets) => {
            asset_storage.extend(assets);
        }
        Err(error) => {
            log::error!("Failed to load assets folder from {}: {:?}", path, error);
        }
    }
}
