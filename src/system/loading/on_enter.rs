use crate::resource::LoadingAssets;
use bevy::prelude::AssetServer;
use bevy::prelude::Res;
use bevy::prelude::ResMut;

pub fn on_enter(asset_server: Res<AssetServer>, mut loading_assets: ResMut<LoadingAssets>) {
    loading_assets
        .assets
        .extend(asset_server.load_folder("actors/").unwrap());

    loading_assets
        .assets
        .extend(asset_server.load_folder("terrain/").unwrap());
}
