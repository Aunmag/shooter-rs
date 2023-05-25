use crate::component::Terrain;
use bevy::{
    ecs::system::Query,
    prelude::{Camera, Transform, With, Without},
};

pub fn terrain(
    cameras: Query<&Transform, With<Camera>>,
    mut terrains: Query<&mut Transform, (With<Terrain>, Without<Camera>)>,
) {
    let count = Terrain::get_count();
    let shift = count * Terrain::SIZE_HALF - Terrain::SIZE_HALF;
    let mut x = 0;
    let mut y = 0;

    if let Some(camera) = cameras.iter().next() {
        x = align_camera(camera.translation.x) - shift;
        y = align_camera(camera.translation.y) - shift;
    }

    for (i, mut terrain) in terrains.iter_mut().enumerate() {
        let i = i as i32;
        terrain.translation.x = (i % count * Terrain::SIZE + x) as f32;
        terrain.translation.y = (i / count * Terrain::SIZE + y) as f32;
    }
}

fn align_camera(n: f32) -> i32 {
    return (n / Terrain::SIZE as f32).round() as i32 * Terrain::SIZE;
}
