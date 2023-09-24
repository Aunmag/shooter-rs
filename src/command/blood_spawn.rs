use crate::{
    data::{LAYER_BLUFF, PIXELS_PER_METER},
    material::BloodMaterial,
    resource::Misc,
};
use bevy::{
    ecs::system::Command,
    prelude::{Assets, Transform, Vec2, Vec3, World},
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
    fn apply(mut self, world: &mut World) {
        self.position = (self.position * PIXELS_PER_METER).floor() / PIXELS_PER_METER;
        let size_px = (self.size * PIXELS_PER_METER / 2.0).floor() * 2.0; // size must be even
        self.size = size_px / PIXELS_PER_METER;

        let time = world.resource::<Time>().elapsed();

        let misc = world.resource::<Misc>();

        let image = if let Some(image) = misc.dummy_image.clone() {
            image
        } else {
            log::warn!("Failed to spawn blood. The dummy image isn't initialized");
            return;
        };

        let mesh = if let Some(mesh) = misc.dummy_mesh.clone() {
            mesh
        } else {
            log::warn!("Failed to spawn blood. The dummy mesh isn't initialized");
            return;
        };

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
