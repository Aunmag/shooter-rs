use amethyst::assets::AssetStorage;
use amethyst::assets::Loader;
use amethyst::assets::ProgressCounter;
use amethyst::ecs::World;
use amethyst::ecs::WorldExt;
use amethyst::renderer::sprite::SpriteSheet;
use amethyst::renderer::sprite::SpriteSheetHandle;
use amethyst::renderer::ImageFormat;
use amethyst::renderer::SpriteSheetFormat;
use amethyst::renderer::Texture;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

const PIXELS_PER_METER: f32 = 32.0;

pub struct SpriteResource {
    data: HashMap<Sprite, SpriteSheetHandle>,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Sprite {
    ActorHuman,
    ActorZombie,
    Bluff,
    BluffCorner,
    Grass,
    Tree0,
    Tree1,
    Tree2,
}

impl SpriteResource {
    pub fn new(world: &World, progress: &mut ProgressCounter) -> Self {
        let mut resource = Self {
            data: HashMap::with_capacity(7),
        };

        let loader = world.read_resource::<Loader>();
        let textures = world.read_resource::<AssetStorage<Texture>>();
        let sprites = world.read_resource::<AssetStorage<SpriteSheet>>();

        resource.load(Sprite::ActorHuman, &loader, &textures, &sprites, progress);
        resource.load(Sprite::ActorZombie, &loader, &textures, &sprites, progress);
        resource.load(Sprite::Bluff, &loader, &textures, &sprites, progress);
        resource.load(Sprite::BluffCorner, &loader, &textures, &sprites, progress);
        resource.load(Sprite::Grass, &loader, &textures, &sprites, progress);
        resource.load(Sprite::Tree0, &loader, &textures, &sprites, progress);
        resource.load(Sprite::Tree1, &loader, &textures, &sprites, progress);
        resource.load(Sprite::Tree2, &loader, &textures, &sprites, progress);

        return resource;
    }

    fn load(
        &mut self,
        sprite: Sprite,
        loader: &Loader,
        textures: &AssetStorage<Texture>,
        sprites: &AssetStorage<SpriteSheet>,
        progress: &mut ProgressCounter,
    ) {
        let path = sprite.get_path();
        let path_to_ron = format!("{}.ron", path);
        let path_to_png = format!("{}.png", path);

        let sprite_sheet = loader.load(
            &path_to_ron,
            SpriteSheetFormat(loader.load(&path_to_png, ImageFormat::default(), (), &textures)),
            progress,
            &sprites,
        );

        self.data.insert(sprite, sprite_sheet);
    }

    pub fn resize_sprites(&self, world: &World) {
        let mut sprite_sheets = world.write_resource::<AssetStorage<SpriteSheet>>();

        for handle in self.data.values() {
            if let Some(sprite_sheet) = sprite_sheets.get_mut(handle) {
                for sprite in sprite_sheet.sprites.iter_mut() {
                    sprite.width /= PIXELS_PER_METER;
                    sprite.height /= PIXELS_PER_METER;

                    for offset in sprite.offsets.iter_mut() {
                        *offset /= PIXELS_PER_METER;
                    }
                }
            }
        }
    }

    pub fn get(&self, sprite: Sprite) -> Option<SpriteSheetHandle> {
        return self.data.get(&sprite).cloned();
    }
}

impl Sprite {
    fn get_path(&self) -> &str {
        return match *self {
            Self::ActorHuman => &"actors/human/image",
            Self::ActorZombie => &"actors/zombie/image",
            Self::Bluff => &"terrain/bluff",
            Self::BluffCorner => &"terrain/bluff_corner",
            Self::Grass => &"terrain/grass",
            Self::Tree0 => &"terrain/tree_0",
            Self::Tree1 => &"terrain/tree_1",
            Self::Tree2 => &"terrain/tree_2",
        };
    }
}
