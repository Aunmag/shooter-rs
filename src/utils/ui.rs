use amethyst::controls::HideCursor;
use amethyst::ecs::Join;
use amethyst::ecs::World;
use amethyst::ecs::WorldExt;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;

pub fn fetch_text(world: &World, id: &'static str) -> Option<String> {
    for (transform, text) in (
        &world.read_storage::<UiTransform>(),
        &world.read_storage::<UiText>(),
    )
        .join()
    {
        if transform.id == id {
            if text.text.is_empty() {
                return None;
            } else {
                return Some(text.text.clone());
            }
        }
    }

    return None;
}

pub fn set_text(world: &World, id: &'static str, text: String) {
    for (transform, ui_text) in (
        &world.read_storage::<UiTransform>(),
        &mut world.write_storage::<UiText>(),
    )
        .join()
    {
        if transform.id == id {
            ui_text.text = text;
            break;
        }
    }
}

pub fn set_button_availability(world: &World, id: &'static str, is_available: bool) {
    let text_id = format!("{}_btn_txt", id);
    let mut is_root_found = false;
    let mut is_text_found = false;

    for (transform, text) in (
        &mut world.write_storage::<UiTransform>(),
        (&mut world.write_storage::<UiText>()).maybe(),
    )
        .join()
    {
        if transform.id == id {
            transform.opaque = is_available;
            is_root_found = true;
        } else if transform.id == text_id {
            if let Some(text) = text {
                if is_available {
                    text.color[3] = 1.0;
                } else {
                    text.color[3] = 0.3;
                }
            }

            is_text_found = true;
        }

        if is_root_found && is_text_found {
            break;
        }
    }
}

pub fn set_cursor_visibility(world: &World, is_visibility: bool) {
    world.write_resource::<HideCursor>().hide = !is_visibility;
}
