use amethyst::assets::AssetStorage;
use amethyst::assets::Loader;
use amethyst::ecs::prelude::World;
use amethyst::ecs::prelude::WorldExt;
use amethyst::renderer::sprite::SpriteSheet;
use amethyst::renderer::sprite::SpriteSheetHandle;
use amethyst::renderer::ImageFormat;
use amethyst::renderer::SpriteSheetFormat;
use amethyst::renderer::Texture;
use std::collections::HashMap;

pub struct SpriteResource {
    pub data: HashMap<Sprite, SpriteSheetHandle>,
}

#[derive(Hash, PartialEq, Eq)]
pub enum Sprite {
    Actor,
    Grass,
}

impl SpriteResource {
    pub fn new(world: &World) -> Self {
        let mut data = HashMap::with_capacity(2);
        let loader = world.read_resource::<Loader>();
        let textures = world.read_resource::<AssetStorage<Texture>>();
        let sprites = world.read_resource::<AssetStorage<SpriteSheet>>();

        data.insert(
            Sprite::Actor,
            Sprite::Actor.load_sprite(&loader, &textures, &sprites),
        );

        data.insert(
            Sprite::Grass,
            Sprite::Grass.load_sprite(&loader, &textures, &sprites),
        );

        return Self { data };
    }

    pub fn get(&self, sprite: &Sprite) -> Option<SpriteSheetHandle> {
        return self.data.get(sprite).cloned();
    }
}

impl Sprite {
    fn load_sprite(
        &self,
        loader: &Loader,
        textures: &AssetStorage<Texture>,
        sprites: &AssetStorage<SpriteSheet>,
    ) -> SpriteSheetHandle {
        let path = self.get_path();
        let path_to_ron = format!("{}.ron", path);
        let path_to_png = format!("{}.png", path);

        return loader.load(
            &path_to_ron,
            SpriteSheetFormat(loader.load(&path_to_png, ImageFormat::default(), (), &textures)),
            (),
            &sprites,
        );
    }

    fn get_path(&self) -> &str {
        return match *self {
            Self::Actor => &"actors/human/image",
            Self::Grass => &"ground/grass",
        };
    }
}
