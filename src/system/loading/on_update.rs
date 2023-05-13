use crate::model::AppState;
use crate::resource::AssetStorage;
use bevy::asset::LoadState;
use bevy::prelude::AssetServer;
use bevy::prelude::NextState;
use bevy::prelude::Res;
use bevy::prelude::ResMut;

pub fn on_update(
    asset_server: Res<AssetServer>,
    asset_storage: Res<AssetStorage>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for asset in asset_storage.iter() {
        match asset_server.get_load_state(asset) {
            LoadState::NotLoaded | LoadState::Loaded | LoadState::Unloaded => {
                // ok
            }
            LoadState::Loading => {
                return; // still loading
            }
            LoadState::Failed => {
                if let Some(path) = asset_server.get_handle_path(asset) {
                    log::error!("Failed to asset from {}", path.path().display());
                }
            }
        }
    }

    next_state.set(AppState::Game);
}
