use crate::resource::{AssetStorage, Cache};
use bevy::{
    prelude::{shape::Quad, AssetServer, Assets, Image, Mesh, Res, ResMut},
    render::render_resource::Extent3d,
};

pub fn on_enter(
    asset_server: Res<AssetServer>,
    mut asset_storage: ResMut<AssetStorage>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
) {
    log::info!("Loading...");
    asset_storage.load(&asset_server);
    init_dummy_image(&mut images, &mut cache);
    init_dummy_mesh(&mut meshes, &mut cache);
}

fn init_dummy_image(images: &mut Assets<Image>, cache: &mut Cache) {
    let mut image = Image::default();
    image.resize(Extent3d {
        width: 1,
        height: 1,
        ..Default::default()
    });

    let handle = images.add(image);
    cache.dummy_image = Some(handle);
}

fn init_dummy_mesh(meshes: &mut Assets<Mesh>, cache: &mut Cache) {
    cache.dummy_mesh = Some(meshes.add(Mesh::from(Quad::default())));
}
