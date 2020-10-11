use crate::utils;
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::World;

mod confirm;
mod home;
mod new_game;
mod quit;

pub use self::confirm::*;
pub use self::home::*;
pub use self::new_game::*;
pub use self::quit::*;

pub trait UiState {
    fn set_visibility(&self, world: &mut World, is_visibility: bool) {
        if let Some(root) = self.get_root() {
            utils::set_entity_visibility(root, world, is_visibility);
        }

        if is_visibility {
            utils::set_cursor_visibility(true, world);
        }
    }

    fn get_root(&self) -> Option<Entity>;
}
