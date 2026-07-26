use crate::{
    plugin::AudioStorage,
    resource::AssetStorage,
    state::AppState,
    util::{ext::AppExt, Timer},
};
use bevy::{
    asset::Assets,
    ecs::system::Local,
    image::Image,
    prelude::{App, AssetServer, AudioSource, IntoScheduleConfigs, NextState, Plugin, Res, ResMut},
    render::mesh::Mesh,
    time::Time,
};
use std::time::Duration;

const INTERVAL: Duration = Duration::from_secs(1);

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(
            AppState::Loading,
            on_update.run_if(|mut r: Local<Timer>, t: Res<Time>| {
                return r.try_next_set(t.elapsed(), || INTERVAL);
            }),
        );
    }
}

fn on_update(
    asset_server: Res<AssetServer>,
    audio_assets: Res<Assets<AudioSource>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_storage: ResMut<AssetStorage>,
    mut audio_storage: ResMut<AudioStorage>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if asset_storage.is_lading_started() {
        if asset_storage.is_loaded(&asset_server) {
            log::info!("Loaded");
            audio_storage.index(&audio_assets, &asset_server);
            next_state.set(AppState::Game);
        } else {
            log::trace!("Loading...");
        }
    } else {
        log::info!("Loading...");
        asset_storage.load(&asset_server, &mut images, &mut meshes);
    }
}
