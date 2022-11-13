use crate::component::Terrain;
use crate::data::LAYER_TERRAIN;
use crate::model::Position;
use bevy::ecs::system::Command;
use bevy::prelude::AssetServer;
use bevy::prelude::SpriteBundle;
use bevy::prelude::World;

pub struct TerrainInit;

impl Command for TerrainInit {
    fn write(self, world: &mut World) {
        let texture = world
            .resource::<AssetServer>()
            .get_handle("terrain/grass.png");

        for _ in 0..Terrain::get_count().pow(2) {
            let _ = world
                .spawn()
                .insert(Terrain)
                .insert_bundle(SpriteBundle {
                    transform: Position::default().to_transform(LAYER_TERRAIN),
                    texture: texture.clone(),
                    ..Default::default()
                })
                .id();
        }
    }
}
