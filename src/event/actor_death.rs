use crate::component::ActorKind;
use bevy::{math::Vec2, prelude::Event};
use derive_more::Constructor;

#[derive(Constructor, Event)]
pub struct ActorDeathEvent {
    pub kind: ActorKind,
    pub position: Vec2,
}
