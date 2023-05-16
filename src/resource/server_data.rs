use bevy::ecs::system::Resource;
use std::time::Duration;

#[derive(Resource)]
pub struct ServerData {
    pub sync_interval: Duration,
}

impl Default for ServerData {
    fn default() -> Self {
        return Self {
            sync_interval: Duration::from_secs(1),
        };
    }
}
