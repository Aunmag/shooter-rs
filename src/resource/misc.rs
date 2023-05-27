use bevy::{asset::Handle, ecs::system::Resource, prelude::Image};

#[derive(Default, Resource)]
pub struct Misc {
    pub dummy_image: Option<Handle<Image>>,
}
