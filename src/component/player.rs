use bevy::ecs::{component::Component, entity::Entity};
use derive_more::Constructor;

#[derive(Constructor, Component)]
pub struct Player {
    pub ghost: Option<Entity>,
}
