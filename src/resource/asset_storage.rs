use bevy::{asset::HandleUntyped, ecs::system::Resource};
use derive_more::{Deref, DerefMut};

#[derive(Default, Resource, Deref, DerefMut)]
pub struct AssetStorage(Vec<HandleUntyped>);
