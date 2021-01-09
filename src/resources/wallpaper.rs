use amethyst::assets::AssetStorage;
use amethyst::assets::Loader;
use amethyst::assets::ProgressCounter;
use amethyst::ecs::prelude::World;
use amethyst::ecs::prelude::WorldExt;
use amethyst::renderer::ImageFormat;
use amethyst::renderer::Texture;
use amethyst::ui::UiImage;
use std::collections::HashMap;

pub struct WallpaperResource {
    data: HashMap<Wallpaper, UiImage>,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Wallpaper {
    Home,
    Quit,
}

impl WallpaperResource {
    pub fn new(world: &World, progress: &mut ProgressCounter) -> Self {
        let mut resource = Self {
            data: HashMap::with_capacity(2),
        };

        let loader = world.read_resource::<Loader>();
        let textures = world.read_resource::<AssetStorage<Texture>>();

        resource.load(Wallpaper::Home, &loader, &textures, progress);
        resource.load(Wallpaper::Quit, &loader, &textures, progress);

        return resource;
    }

    fn load(
        &mut self,
        wallpaper: Wallpaper,
        loader: &Loader,
        textures: &AssetStorage<Texture>,
        progress: &mut ProgressCounter,
    ) {
        let texture = UiImage::Texture(loader.load(
            wallpaper.get_path(),
            ImageFormat::default(),
            progress,
            &textures,
        ));

        self.data.insert(wallpaper, texture);
    }

    pub fn get(&self, sprite: Wallpaper) -> Option<UiImage> {
        return self.data.get(&sprite).cloned();
    }
}

impl Wallpaper {
    fn get_path(&self) -> &str {
        return match *self {
            Self::Home => &"wallpapers/home.png",
            Self::Quit => &"wallpapers/quit.png",
        };
    }
}
