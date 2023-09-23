use crate::component::ActorKind;
use bevy::math::Vec2;
use derive_more::Constructor;

#[derive(Constructor)]
pub struct ActorDeathEvent {
    pub kind: ActorKind,
    pub position: Vec2,
}
