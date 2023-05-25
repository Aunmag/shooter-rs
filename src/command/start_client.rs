use crate::resource::ServerData;
use bevy::{ecs::system::Command, prelude::World};
use std::time::Duration;

pub struct StartClient {
    pub sync_interval: Duration,
}

impl Command for StartClient {
    fn write(self, world: &mut World) {
        world.resource_mut::<ServerData>().sync_interval = self.sync_interval;
    }
}
