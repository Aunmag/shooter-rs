use crate::{component::Terrain, data::LAYER_TERRAIN, model::TransformLite};
use bevy::{
    ecs::system::Command,
    prelude::{AssetServer, SpriteBundle, World},
};

pub struct TerrainInit;

impl Command for TerrainInit {
    fn write(self, world: &mut World) {
        let texture = world
            .resource::<AssetServer>()
            .get_handle("terrain/grass.png");

        for _ in 0..Terrain::get_count().pow(2) {
            world
                .spawn(SpriteBundle {
                    transform: TransformLite::default().as_transform(LAYER_TERRAIN),
                    texture: texture.clone(),
                    ..Default::default()
                })
                .insert(Terrain);
        }
    }
}
