use crate::{
    data::{LAYER_BLUFF, PIXELS_PER_METER},
    material::BloodMaterial,
    resource::{Cache, Config},
};
use bevy::{
    asset::Handle,
    ecs::{entity::Entity, system::Command},
    prelude::{Assets, DespawnRecursiveExt, Transform, Vec2, Vec3, World},
    sprite::MaterialMesh2dBundle,
    time::Time,
};
use rand::{thread_rng, Rng};

pub struct BloodSpawn {
    position: Vec2,
    size: f32,
    size_px: f32,
}

impl BloodSpawn {
    /// NOTE: Visual blood size is always smaller than it's mesh size
    const SIZE_MIN_PX: f32 = 8.0;

    pub fn new(mut position: Vec2, mut size: f32) -> Self {
        position = (position * PIXELS_PER_METER).floor() / PIXELS_PER_METER;
        let size_px = (size * PIXELS_PER_METER / 2.0).floor() * 2.0; // size must be even
        size = size_px / PIXELS_PER_METER;

        return Self {
            position,
            size,
            size_px,
        };
    }

    pub fn is_too_small(&self) -> bool {
        return self.size_px < Self::SIZE_MIN_PX;
    }
}

impl Command for BloodSpawn {
    fn apply(self, world: &mut World) {
        let cache = world.resource::<Cache>();

        let Some(image) = cache.dummy_image.clone() else {
            log::warn!("Failed to spawn blood. The dummy image isn't initialized");
            return;
        };

        let Some(mesh) = cache.dummy_mesh.clone() else {
            log::warn!("Failed to spawn blood. The dummy mesh isn't initialized");
            return;
        };

        if !reserve_decal(world) {
            return;
        }

        let time = world.resource::<Time>().elapsed();

        let material = world
            .resource_mut::<Assets<BloodMaterial>>()
            .add(BloodMaterial {
                spawned: time,
                seed: thread_rng().gen_range(0.0..500.0),
                size: self.size_px,
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

fn reserve_decal(world: &mut World) -> bool {
    let mut decals = 0;
    let mut smallest = None;
    let mut smallest_size = f32::INFINITY;

    let mut query = world.query::<(Entity, &Handle<BloodMaterial>)>();
    let assets = world.resource::<Assets<BloodMaterial>>();

    for (entity, handle) in query.iter(world) {
        decals += 1;

        if let Some(material) = assets.get(handle) {
            if material.size < smallest_size {
                smallest = Some(entity);
                smallest_size = material.size;
            }
        }
    }

    if decals >= world.resource::<Config>().graphic.decals {
        if let Some(smallest) = smallest {
            world.entity_mut(smallest).despawn_recursive();
            return true;
        } else {
            return false;
        }
    } else {
        return true;
    }
}
