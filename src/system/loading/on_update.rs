use crate::{
    model::AppState,
    resource::{AssetGroups, AssetStorage},
    util::Timer,
};
use bevy::{
    asset::Assets,
    ecs::{schedule::SystemConfigs, system::Local},
    prelude::{AssetServer, AudioSource, IntoSystemConfigs, NextState, Res, ResMut},
    render::{mesh::Mesh, texture::Image},
    time::Time,
};
use std::time::Duration;

const INTERVAL: Duration = Duration::from_secs(1);

fn on_update_inner(
    asset_server: Res<AssetServer>,
    audio_assets: Res<Assets<AudioSource>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_storage: ResMut<AssetStorage>,
    mut audio_storage: ResMut<AssetGroups<AudioSource>>,
    mut image_storage: ResMut<AssetGroups<Image>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if asset_storage.is_lading_started() {
        if asset_storage.is_loaded(&asset_server) {
            log::info!("Loaded");
            audio_storage.index(&audio_assets, &asset_server, true);
            image_storage.index(&images, &asset_server, false);
            next_state.set(AppState::Game);
        } else {
            log::trace!("Loading...");
        }
    } else {
        log::info!("Loading...");
        asset_storage.load(&asset_server, &mut images, &mut meshes);
    }
}

pub fn on_update() -> SystemConfigs {
    return on_update_inner.run_if(|mut r: Local<Timer>, t: Res<Time>| {
        return r.next_if_ready(t.elapsed(), || INTERVAL);
    });
}
