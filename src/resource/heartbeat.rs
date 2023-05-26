use bevy::{
    ecs::system::Resource,
    prelude::{AudioSink, Handle},
};
use std::time::Duration;

#[derive(Default, Resource)]
pub struct HeartbeatResource {
    pub sink: Option<Handle<AudioSink>>,
    pub next: Duration,
}
