use bevy::ecs::system::Command;
use bevy::prelude::World;

pub struct Start;

impl Command for Start {
    fn write(self, _: &mut World) {
        // TODO: implement later when menu sate will be used to unpause the game
    }
}
