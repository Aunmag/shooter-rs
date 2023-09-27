use crate::component::ActorKind;
use bevy::{ecs::entity::Entity, math::Vec2, prelude::Event};
use derive_more::Constructor;

#[derive(Constructor, Event)]
pub struct ActorDeathEvent {
    pub entity: Entity,
    pub kind: ActorKind,
    pub position: Vec2,
    pub attacker: Option<Entity>,
}
