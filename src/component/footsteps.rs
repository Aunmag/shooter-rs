use bevy::ecs::component::Component;
use bevy::math::Vec2;
use std::time::Duration;

#[derive(Default, Component)]
pub struct Footsteps {
    pub position: Vec2,
    pub time: Duration,
}
