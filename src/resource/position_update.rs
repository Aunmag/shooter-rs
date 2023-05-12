use crate::model::Position;
use bevy::ecs::system::Resource;
use derive_more::Deref;
use derive_more::DerefMut;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct PositionUpdateResource(Vec<(u32, Position)>); // TODO: avoid duplicates
