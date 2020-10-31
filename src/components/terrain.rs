use amethyst::core::math::Point3;
use amethyst::ecs::prelude::World;
use amethyst::tiles::Tile;

#[derive(Default, Clone)]
pub struct Terrain;

impl Terrain {
    pub const SIZE: u32 = 4;
    pub const SIZE_HALF: f32 = Self::SIZE as f32 / 2.0;
    pub const QUANTITY: u32 = 5;
}

impl Tile for Terrain {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        return Some(0);
    }
}
