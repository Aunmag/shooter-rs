use bevy::asset::HandleUntyped;
use bevy::ecs::system::Resource;
use derive_more::Deref;
use derive_more::DerefMut;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct AssetStorage(Vec<HandleUntyped>);
