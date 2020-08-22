use amethyst::assets::AssetStorage;
use amethyst::assets::Loader;
use amethyst::controls::HideCursor;
use amethyst::core::HiddenPropagate;
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::World;
use amethyst::prelude::*;
use amethyst::renderer::sprite::SpriteSheetHandle;
use amethyst::renderer::ImageFormat;
use amethyst::renderer::SpriteSheet;
use amethyst::renderer::SpriteSheetFormat;
use amethyst::renderer::Texture;

pub fn set_cursor_visibility(is_visible: bool, world: &mut World) {
    world.write_resource::<HideCursor>().hide = !is_visible;
}

pub fn set_entity_visibility(entity: Entity, world: &mut World, is_visible: bool) {
    let mut storage = world.write_storage::<HiddenPropagate>();

    if is_visible {
        if storage.remove(entity).is_none() {
            log::warn!(
                "There was no HiddenPropagate component to delete from {:?}",
                entity,
            );
        }
    } else if let Err(error) = storage.insert(entity, HiddenPropagate::new()) {
        log::error!(
            "Failed to insert HiddenPropagate component. Details: {}",
            error,
        );
    }
}

/// Workaround utility to wait `UiAwaiter::FRAMES_TO_AWAIT` frames for UI initialization
pub struct UiAwaiter {
    frames_passed: u8,
}

impl UiAwaiter {
    /// Usually it takes 4 frames to initialize all the UI entities, but we'll wait a little more just in case
    const FRAMES_TO_AWAIT: u8 = 16;

    pub fn new() -> Self {
        return Self { frames_passed: 0 };
    }

    pub fn update(&mut self) {
        if self.frames_passed < Self::FRAMES_TO_AWAIT {
            self.frames_passed += 1;
        }
    }

    pub fn is_ready(&self) -> bool {
        return self.frames_passed >= Self::FRAMES_TO_AWAIT;
    }
}

pub fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    return world.read_resource::<Loader>().load(
        ron_path,
        SpriteSheetFormat(world.read_resource::<Loader>().load(
            png_path,
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )),
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    );
}
