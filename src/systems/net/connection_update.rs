use crate::resources::NetResource;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::shred::WriteExpect;

#[derive(SystemDesc)]
pub struct ConnectionUpdateSystem;

impl<'a> System<'a> for ConnectionUpdateSystem {
    type SystemData = (WriteExpect<'a, NetResource>,);

    fn run(&mut self, (mut net,): Self::SystemData) {
        net.update_connections();
    }
}
