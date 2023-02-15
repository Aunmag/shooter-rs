use crate::resource::LoadingAssets;
use bevy::prelude::AssetServer;
use bevy::prelude::Res;
use bevy::prelude::ResMut;

pub fn on_enter(asset_server: Res<AssetServer>, mut loading_assets: ResMut<LoadingAssets>) {
    load_folder_or_log(&asset_server, &mut loading_assets, "actors/");
    load_folder_or_log(&asset_server, &mut loading_assets, "terrain/");
}

fn load_folder_or_log(
    asset_server: &Res<AssetServer>,
    loading_assets: &mut ResMut<LoadingAssets>,
    path: &str,
) {
    match asset_server.load_folder(path) {
        Ok(assets) => {
            loading_assets.assets.extend(assets);
        }
        Err(error) => {
            log::error!("Failed to load assets folder from {}: {:?}", path, error);
        }
    }
}
