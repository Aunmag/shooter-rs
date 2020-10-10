use crate::utils;
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::World;
use amethyst::ui::UiFinder;

mod confirm;
mod home;
mod new_game;
mod quit;

pub use self::confirm::*;
pub use self::home::*;
pub use self::new_game::*;
pub use self::quit::*;

pub trait UiState {
    fn on_start_or_resume(&self, world: &mut World) {
        self.set_visibility(true, world);
        utils::set_cursor_visibility(true, world);
    }

    fn on_stop_or_pause(&self, world: &mut World) {
        self.set_visibility(false, world);
    }

    fn find_ui_root(&self, world: &mut World) -> Option<Entity> {
        return world.exec(|f: UiFinder| f.find(self.get_ui_root_id()));
    }

    fn set_visibility(&self, is_visible: bool, world: &mut World) {
        if let Some(ui_root) = self.get_ui_root() {
            utils::set_entity_visibility(ui_root, world, is_visible);
        }
    }

    fn get_ui_root_id(&self) -> &'static str;

    fn get_ui_root(&self) -> Option<Entity>;
}
