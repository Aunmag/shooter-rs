use crate::model::AppState;
use crate::resource::LoadingAssets;
use bevy::asset::LoadState;
use bevy::prelude::AssetServer;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::State;

pub fn on_update(
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>, // TODO: try without it
    mut state: ResMut<State<AppState>>,
) {
    let mut is_loaded = true;

    for asset in loading_assets.iter() {
        if let LoadState::Loading = asset_server.get_load_state(asset) {
            is_loaded = false;
            break;
        }
    }

    if is_loaded {
        loading_assets.clear();

        if let Err(error) = state.set(AppState::Game) {
            log::error!("Failed to set state: {:?}", error);
        }

        // TODO: remove resource `LoadingAssets`
    }
}
