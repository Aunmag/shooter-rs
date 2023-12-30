use bevy::{app::AppExit, ecs::system::Command, prelude::World};

pub struct Exit;

impl Command for Exit {
    fn apply(self, world: &mut World) {
        world.send_event(AppExit);
    }
}
