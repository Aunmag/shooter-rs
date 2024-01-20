use crate::{
    data::{LAYER_BLUFF, PIXELS_PER_METER},
    material::BloodMaterial,
    resource::{AssetStorage, Config},
    util::math::{floor_by, interpolate_unbounded},
};
use bevy::{
    ecs::{component::Component, entity::Entity, query::With, system::Command},
    prelude::{Assets, DespawnRecursiveExt, Transform, Vec2, Vec3, World},
    render::camera::Camera,
    sprite::MaterialMesh2dBundle,
    time::Time,
};
use rand::{thread_rng, Rng};

const SIZE_MIN: f32 = 0.8;
const SIZE_MAX: f32 = 6.0;
const SCALE_MIN: f32 = 0.03;

pub struct BloodSpawn {
    position: Vec2,
    size: f32,
    size_px: f32,
}

impl BloodSpawn {
    pub fn new(mut position: Vec2, mut scale: f32) -> Option<Self> {
        scale = scale.clamp(0.0, 1.0);

        if scale < SCALE_MIN {
            return None;
        }

        let size_raw = interpolate_unbounded(SIZE_MIN, SIZE_MAX, scale);

        position = (position * PIXELS_PER_METER).floor() / PIXELS_PER_METER;
        let size_px = floor_by(size_raw * PIXELS_PER_METER, 2.0); // size must be even
        let size = size_px / PIXELS_PER_METER;

        return Some(Self {
            position,
            size,
            size_px,
        });
    }
}

impl Command for BloodSpawn {
    fn apply(self, world: &mut World) {
        if !reserve_decal(world) {
            return;
        }

        let assets = world.resource::<AssetStorage>();
        let image = assets.dummy_image().clone();
        let mesh = assets.dummy_mesh().clone();
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

        world
            .spawn(MaterialMesh2dBundle {
                transform: Transform {
                    translation: self.position.extend(LAYER_BLUFF),
                    scale: Vec3::splat(self.size),
                    ..Transform::default()
                },
                mesh: mesh.into(),
                material,
                ..Default::default()
            })
            .insert(BloodComponent);
    }
}

fn reserve_decal(world: &mut World) -> bool {
    let camera = world
        .query_filtered::<&Transform, With<Camera>>()
        .iter(world)
        .next()
        .map(|t| t.translation.truncate());

    let mut decals = 0;
    let mut furthest = None;
    let mut furthest_distance = 0.0;

    for (entity, transform) in world
        .query_filtered::<(Entity, &Transform), With<BloodComponent>>()
        .iter(world)
    {
        decals += 1;

        if let Some(camera) = camera {
            let distance = camera.distance(transform.translation.truncate());

            if distance > furthest_distance {
                furthest = Some(entity);
                furthest_distance = distance;
            }
        } else if furthest.is_none() {
            furthest = Some(entity);
        }
    }

    if decals >= world.resource::<Config>().graphic.decals {
        if let Some(oldest) = furthest {
            world.entity_mut(oldest).despawn_recursive();
            return true;
        } else {
            return false;
        }
    } else {
        return true;
    }
}

#[derive(Component)]
struct BloodComponent;
