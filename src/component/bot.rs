use bevy::ecs::component::Component;
use bevy::math::Vec2;
use bevy::prelude::Entity;

#[derive(Default, Component)]
pub struct Bot {
    pub target_actor: Option<Entity>,
    pub target_final: Option<Vec2>,
}
