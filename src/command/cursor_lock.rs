use crate::util::ext::WorldExt;
use bevy::ecs::system::Command;
use bevy::prelude::Windows;
use bevy::prelude::World;
use bevy::window::CursorGrabMode;

pub struct CursorGrab(pub bool);

impl CursorGrab {
    pub fn get_mode(&self) -> CursorGrabMode {
        if self.0 {
            return CursorGrabMode::Confined;
        } else {
            return CursorGrabMode::None;
        }
    }
}

impl Command for CursorGrab {
    fn write(self, world: &mut World) {
        if !self.0 || world.config().misc.grab_cursor {
            if let Some(window) = world.resource_mut::<Windows>().get_primary_mut() {
                window.set_cursor_grab_mode(self.get_mode());
                window.set_cursor_visibility(!self.0);
            }
        }
    }
}
