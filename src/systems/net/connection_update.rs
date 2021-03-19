use crate::resources::NetResource;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::Write;

pub struct ConnectionUpdateSystem;

impl<'a> System<'a> for ConnectionUpdateSystem {
    type SystemData = (Option<Write<'a, NetResource>>,);

    fn run(&mut self, (net,): Self::SystemData) {
        let mut net = match net {
            Some(net) => net,
            None => return,
        };

        net.update_connections();
    }
}
