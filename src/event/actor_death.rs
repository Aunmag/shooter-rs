use crate::component::ActorType;
use bevy::math::Vec2;
use derive_more::Constructor;

#[derive(Constructor)]
pub struct ActorDeathEvent {
    pub actor_type: ActorType,
    pub position: Vec2,
}
