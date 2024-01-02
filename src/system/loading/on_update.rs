use crate::{
    model::AppState,
    resource::{AssetStorage, AudioStorage},
    util::Timer,
};
use bevy::{
    asset::Assets,
    ecs::{schedule::SystemConfigs, system::Local},
    prelude::{AssetServer, AudioSource, IntoSystemConfigs, NextState, Res, ResMut},
    time::Time,
};
use std::time::Duration;

const INTERVAL: Duration = Duration::from_secs(1);

fn on_update_inner(
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

pub fn on_update() -> SystemConfigs {
    return on_update_inner.run_if(|mut r: Local<Timer>, t: Res<Time>| {
        return r.next_if_ready(t.elapsed(), || INTERVAL);
    });
}
