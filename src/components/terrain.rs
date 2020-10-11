use crate::data::LAYER_TERRAIN;
use crate::utils;
use amethyst::core::math::Point3;
use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::core::Parent;
use amethyst::ecs::prelude::World;
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::tiles::MortonEncoder;
use amethyst::tiles::Tile;
use amethyst::tiles::TileMap;

#[derive(Default, Clone)]
pub struct Terrain;

impl Terrain {
    pub const SIZE: u32 = 128;
    pub const SIZE_HALF: f32 = Self::SIZE as f32 / 2.0;
    pub const QUANTITY: u32 = 5;

    pub fn create_entity(world: &mut World, root: Entity) -> Entity {
        let tile_map = TileMap::<Self, MortonEncoder>::new(
            Vector3::new(Self::QUANTITY, Self::QUANTITY, 1),
            Vector3::new(Self::SIZE, Self::SIZE, 1),
            Some(utils::load_sprite_sheet(
                world,
                "ground/grass.png",
                "ground/grass.ron",
            )),
        );

        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 0.0, LAYER_TERRAIN);

        return world
            .create_entity()
            .with(Parent { entity: root })
            .with(tile_map)
            .with(transform)
            .build();
    }
}

impl Tile for Terrain {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        return Some(0);
    }
}
