use bevy::{ecs::component::Component, math::Vec2, prelude::Entity};
use std::time::Duration;

#[derive(Default, Component)]
pub struct Bot {
    pub target_actor: Option<Entity>,
    pub target_final: Option<Vec2>,
    pub next_sound: Duration,
}
