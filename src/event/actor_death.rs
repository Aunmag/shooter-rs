use bevy::math::Vec2;
use derive_more::Constructor;

#[derive(Constructor)]
pub struct ActorDeathEvent {
    pub position: Vec2,
}
