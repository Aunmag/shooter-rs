use bevy::ecs::component::Component;
use derive_more::{Constructor, Deref};
use std::time::Duration;

#[derive(Component, Constructor, Deref)]
pub struct AudioExpiration(Duration);
