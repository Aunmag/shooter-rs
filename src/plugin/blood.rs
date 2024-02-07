use crate::{
    data::{LAYER_BLUFF, PIXELS_PER_METER},
    model::AppState,
    resource::{AssetStorage, Config},
    util::{
        ext::{AppExt, DurationExt},
        math::{floor_by, interpolate_unbounded},
    },
};
use bevy::{
    app::{App, Plugin},
    asset::Asset,
    ecs::{
        entity::Entity,
        query::With,
        system::{Command, ResMut},
    },
    prelude::{Assets, DespawnRecursiveExt, Handle, Image, Res, Transform, Vec2, Vec3, World},
    reflect::TypePath,
    render::{
        camera::Camera,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    time::Time,
};
use rand::{thread_rng, Rng};
use std::time::Duration;

const SIZE_MIN: f32 = 0.8;
const SIZE_MAX: f32 = 6.0;
const SCALE_MIN: f32 = 0.03;

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
        .query_filtered::<(Entity, &Transform), With<Handle<Blood>>>()
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

const SPREAD_DURATION: Duration = Duration::from_millis(150);

pub struct BloodPlugin;

impl Plugin for BloodPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<Blood>::default());
        app.add_state_system(AppState::Game, on_update);
    }
}

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
pub struct Blood {
    spawned: Duration,
    #[uniform(0)]
    seed: f32,
    #[uniform(0)]
    size: f32,
    #[uniform(0)]
    spread: f32,
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
}

impl Material2d for Blood {
    fn fragment_shader() -> ShaderRef {
        return "shader/blood.wgsl".into();
    }
}

fn on_update(mut materials: ResMut<Assets<Blood>>, time: Res<Time>) {
    let time = time.elapsed();

    for (_, material) in materials.iter_mut() {
        material.spread = time.progress(material.spawned, material.spawned + SPREAD_DURATION);
    }
}

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

        let material = world.resource_mut::<Assets<Blood>>().add(Blood {
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
