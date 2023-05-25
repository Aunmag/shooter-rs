use crate::{
    model::AppState,
    resource::{AssetStorage, AudioStorage},
};
use bevy::{
    asset::LoadState,
    prelude::{AssetServer, Assets, AudioSource, NextState, Res, ResMut},
};

pub fn on_update(
    asset_server: Res<AssetServer>,
    asset_storage: Res<AssetStorage>,
    assets_audio: Res<Assets<AudioSource>>,
    mut audio_storage: ResMut<AudioStorage>,
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

    audio_storage.index(&assets_audio, &asset_server);
    next_state.set(AppState::Game);
}
