use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::{App, Plugin, *},
    reflect::TypePath,
};

/// Just allows to skip (ignore) certain types of files in assets directory
pub struct SkipLoaderPlugin;

impl Plugin for SkipLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SkipAsset>();
        app.init_asset_loader::<SkipLoader>();
    }
}

#[derive(Asset, TypePath)]
struct SkipAsset;

#[derive(Default, TypePath)]
struct SkipLoader;

impl AssetLoader for SkipLoader {
    type Asset = SkipAsset;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        _: &mut dyn Reader,
        _: &(),
        _: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        return Ok(SkipAsset);
    }

    fn extensions(&self) -> &[&str] {
        return &["txt"];
    }
}
