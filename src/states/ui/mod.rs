mod confirm;
mod home;
mod loading;
mod new_game;

pub use self::confirm::*;
pub use self::home::*;
pub use self::loading::*;
pub use self::new_game::*;
use crate::resources::Wallpaper;
use crate::resources::WallpaperResource;
use crate::utils;
use crate::utils::WorldExtCustom;
use amethyst::core::ecs::Join;
use amethyst::core::Parent;
use amethyst::ecs::Entity;
use amethyst::ecs::World;
use amethyst::ecs::WorldExt;
use amethyst::ui::UiTransform;

const WALLPAPER_ID: &str = "wallpaper";

pub trait UiState {
    fn set_visibility(&self, world: &World, is_visibility: bool) {
        if let Some(root) = self.get_root() {
            utils::set_entity_visibility(world, root, is_visibility);
        }

        if is_visibility {
            utils::ui::set_cursor_visibility(world, true);
        }
    }

    fn set_wallpaper(&self, world: &World, wallpaper: Wallpaper) {
        if let Some(root) = self.get_root() {
            if let Some(image) = world.read_resource::<WallpaperResource>().get(wallpaper) {
                for (entity, parent, transform) in (
                    &world.entities(),
                    &world.read_storage::<Parent>(),
                    &world.read_storage::<UiTransform>(),
                )
                    .join()
                {
                    if parent.entity == root && transform.id == WALLPAPER_ID {
                        world.add(entity, image);
                        break;
                    }
                }
            }
        }
    }

    fn get_root(&self) -> Option<Entity>;
}
