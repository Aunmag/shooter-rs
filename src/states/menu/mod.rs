use crate::utils;
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::World;
use amethyst::ecs::prelude::WriteStorage;
use amethyst::ui::UiFinder;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;
use std::collections::HashSet;

pub mod home;
pub mod quit;

pub use self::home::*;
pub use self::quit::*;

const FONT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const FONT_COLOR_DISABLED: [f32; 4] = [0.8, 0.8, 0.8, 0.5];

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

pub fn set_buttons_availability(
    ids: &[&str],
    is_availability: bool,
    transforms: &mut WriteStorage<UiTransform>,
    texts: &mut WriteStorage<UiText>,
) {
    let mut transforms_id = HashSet::with_capacity(ids.len());
    let mut texts_id = HashSet::with_capacity(ids.len());

    for id in ids {
        transforms_id.insert((*id).to_string());
        texts_id.insert(format!("{}_btn_txt", id));
    }

    for (transform, text) in (transforms, texts.maybe()).join() {
        if transforms_id.contains(&transform.id) {
            transform.opaque = is_availability;
        }

        if let Some(text) = text {
            if texts_id.contains(&transform.id) {
                if is_availability {
                    text.color = FONT_COLOR;
                } else {
                    text.color = FONT_COLOR_DISABLED;
                }
            }
        }
    }
}
