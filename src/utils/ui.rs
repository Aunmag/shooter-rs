use amethyst::ecs::prelude::Join;
use amethyst::prelude::World;
use amethyst::prelude::WorldExt;
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
            if text.text == "" {
                return None;
            } else {
                return Some(text.text.clone());
            }
        }
    }

    return None;
}
