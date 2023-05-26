use bevy::ecs::component::Component;
use std::time::Duration;

#[derive(Default, Component)]
pub struct Breath {
    pub last: Duration,
}
