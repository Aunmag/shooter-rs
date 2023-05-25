use crate::model::TransformLiteU8;
use bevy::ecs::system::Resource;
use derive_more::{Deref, DerefMut};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct TransformUpdateResource(Vec<(u32, TransformLiteU8)>); // TODO: avoid duplicates
