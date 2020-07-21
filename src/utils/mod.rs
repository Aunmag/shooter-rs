pub mod input;
pub mod math;

use amethyst::controls::HideCursor;
use amethyst::core::HiddenPropagate;
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::World;
use amethyst::prelude::*;

pub fn set_cursor_visibility(is_visible: bool, world: &mut World) {
    world.write_resource::<HideCursor>().hide = !is_visible;
}

pub fn set_entity_visibility(entity: Entity, world: &mut World, is_visible: bool) {
    // TODO: Do not use `expect`, write warning to log

    let mut storage = world.write_storage::<HiddenPropagate>();

    if is_visible {
        storage
            .remove(entity)
            .expect(&"Failed to delete HiddenPropagate component");
    } else {
        storage
            .insert(entity, HiddenPropagate::new())
            .expect(&"Failed to insert HiddenPropagate component");
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
