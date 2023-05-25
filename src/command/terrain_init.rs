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

        #[allow(clippy::let_underscore_must_use)] // don't know why it must be used
        for _ in 0..Terrain::get_count().pow(2) {
            let _ = world
                .spawn(SpriteBundle {
                    transform: TransformLite::default().as_transform(LAYER_TERRAIN),
                    texture: texture.clone(),
                    ..Default::default()
                })
                .insert(Terrain)
                .id();
        }
    }
}
