use crate::component::ActorKind;
use bevy::{math::Vec2, prelude::Event};

#[derive(Event)]
pub struct ActorDeathEvent {
    pub kind: ActorKind,
    pub position: Vec2,
    pub is_player: bool,
}
