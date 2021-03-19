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

pub fn set_cursor_visibility(world: &World, is_visibility: bool) {
    world.write_resource::<HideCursor>().hide = !is_visibility;
}
