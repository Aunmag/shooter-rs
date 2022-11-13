use crate::data::VIEW_DISTANCE;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Terrain;

impl Terrain {
    pub const SIZE_HALF: i32 = 2;
    pub const SIZE: i32 = Self::SIZE_HALF * 2;

    pub fn get_count() -> i32 {
        return (VIEW_DISTANCE / Terrain::SIZE as f32 + 0.5).ceil() as i32;
    }
}
