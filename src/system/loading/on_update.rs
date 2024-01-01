use crate::{
    model::AppState,
    resource::{AssetStorage, AudioStorage},
};
use bevy::{
    asset::Assets,
    prelude::{AssetServer, AudioSource, NextState, Res, ResMut},
};

pub fn on_update(
    asset_server: Res<AssetServer>,
    audio_assets: Res<Assets<AudioSource>>,
    mut asset_storage: ResMut<AssetStorage>,
    mut audio_storage: ResMut<AudioStorage>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match asset_storage.is_loaded(&asset_server) {
        None => {
            log::info!("Loading...");
            asset_storage.load(&asset_server);
            return;
        }
        Some(false) => {
            log::trace!("Loading...");
            return;
        }
        Some(true) => {
            log::info!("Loaded");
            audio_storage.index(&audio_assets, &asset_server);
            next_state.set(AppState::Game);
        }
    }
}
