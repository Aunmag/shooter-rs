use crate::resource::{AssetStorage, Misc};
use bevy::{
    prelude::{AssetServer, Assets, Image, Res, ResMut},
    render::render_resource::Extent3d,
};

pub fn on_enter(
    asset_server: Res<AssetServer>,
    mut asset_storage: ResMut<AssetStorage>,
    mut images: ResMut<Assets<Image>>,
    mut misc: ResMut<Misc>,
) {
    load_folder_or_log(&asset_server, &mut asset_storage, "actors");
    load_folder_or_log(&asset_server, &mut asset_storage, "fonts");
    load_folder_or_log(&asset_server, &mut asset_storage, "sounds");
    load_folder_or_log(&asset_server, &mut asset_storage, "terrain");
    load_folder_or_log(&asset_server, &mut asset_storage, "weapons");
    init_dummy_image(&mut images, &mut misc);
}

fn load_folder_or_log(
    asset_server: &Res<AssetServer>,
    asset_storage: &mut ResMut<AssetStorage>,
    path: &str,
) {
    match asset_server.load_folder(path) {
        Ok(assets) => {
            asset_storage.extend(assets);
        }
        Err(error) => {
            log::error!("Failed to load assets folder from {}: {:?}", path, error);
        }
    }
}

fn init_dummy_image(images: &mut Assets<Image>, misc: &mut Misc) {
    let mut image = Image::default();
    image.resize(Extent3d {
        width: 1,
        height: 1,
        ..Default::default()
    });

    let handle = images.add(image);
    misc.dummy_image = Some(handle);
}
