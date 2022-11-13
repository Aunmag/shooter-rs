use crate::util::ext::WorldExt;
use bevy::ecs::system::Command;
use bevy::prelude::Windows;
use bevy::prelude::World;

pub struct CursorLock(pub bool);

impl Command for CursorLock {
    fn write(self, world: &mut World) {
        if !self.0 || world.config().misc.lock_cursor {
            if let Some(window) = world.resource_mut::<Windows>().get_primary_mut() {
                window.set_cursor_lock_mode(self.0);
                window.set_cursor_visibility(!self.0);
            }
        }
    }
}
