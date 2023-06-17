use crate::{
    data::{LAYER_BLUFF, PIXELS_PER_METER},
    material::BloodMaterial,
    resource::Misc,
};
use bevy::{
    ecs::system::Command,
    prelude::{shape::Cube, Assets, Mesh, Transform, Vec2, Vec3, World},
    sprite::MaterialMesh2dBundle,
    time::Time,
};
use derive_more::Constructor;
use rand::{thread_rng, Rng};

#[derive(Constructor)]
pub struct BloodSpawn {
    position: Vec2,
    size: f32,
}

impl Command for BloodSpawn {
    fn write(mut self, world: &mut World) {
        self.position.x = (self.position.x * PIXELS_PER_METER).floor() / PIXELS_PER_METER;
        self.position.y = (self.position.y * PIXELS_PER_METER).floor() / PIXELS_PER_METER;
        let size_px = (self.size * PIXELS_PER_METER / 2.0).floor() * 2.0; // size must be even
        self.size = size_px / PIXELS_PER_METER;

        let time = world.resource::<Time>().elapsed();

        let image = if let Some(image) = world.resource_mut::<Misc>().dummy_image.clone() {
            image
        } else {
            log::warn!("Failed spawn blood. The dummy image isn't initialized");
            return;
        };

        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Mesh::from(Cube::default()));

        let material = world
            .resource_mut::<Assets<BloodMaterial>>()
            .add(BloodMaterial {
                spawned: time,
                seed: thread_rng().gen_range(0.0..500.0),
                size: size_px,
                spread: 0.0,
                image,
            });

        world.spawn(MaterialMesh2dBundle {
            transform: Transform {
                translation: self.position.extend(LAYER_BLUFF),
                scale: Vec3::splat(self.size),
                ..Transform::default()
            },
            mesh: mesh.into(),
            material,
            ..Default::default()
        });
    }
}
