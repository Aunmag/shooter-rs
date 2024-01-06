use crate::{
    command::CursorGrab,
    component::Terrain,
    data::{
        LAYER_BLUFF, LAYER_TERRAIN, LAYER_TREE, TRANSFORM_SCALE, WORLD_SIZE, WORLD_SIZE_HALF,
        WORLD_SIZE_VISUAL,
    },
    model::{AudioPlay, TransformLite},
    resource::AudioTracker,
    util::ext::Vec2Ext,
};
use bevy::{
    asset::AssetServer,
    ecs::{system::Command, world::World},
    math::{Quat, Vec2, Vec3},
    prelude::{Camera2dBundle, SpriteBundle},
    transform::components::Transform,
};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::{
    f32::consts::{FRAC_PI_2, PI, TAU},
    time::Duration,
};

const TREES_PER_METER: f32 = 0.02;
const TREES_QUANTITY: f32 = WORLD_SIZE_VISUAL * WORLD_SIZE_VISUAL * TREES_PER_METER;
const TREE_BUFFER_ZONE: f32 = 3.2;
const TREE_FIND_POSITION_ATTEMPTS: usize = 32;
const BLUFF_SPRITE_SIZE: f32 = 4.0;

pub fn on_enter(world: &mut World) {
    CursorGrab(true).apply(world);
    world.spawn(Camera2dBundle::default());
    spawn_terrain(world);
    spawn_bluffs(world);
    spawn_trees(world);
    play_audio(world.resource::<AudioTracker>());
}

fn spawn_terrain(world: &mut World) {
    let texture = world
        .resource::<AssetServer>()
        .get_handle("terrain/grass.png")
        .unwrap_or_default();

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

// TODO: maybe render bluff corner as tile map
fn spawn_bluffs(world: &mut World) {
    let n = WORLD_SIZE_HALF;
    let z = LAYER_BLUFF;
    let r1 = PI;
    let r2 = 0.0;
    let r3 = FRAC_PI_2;
    let r4 = FRAC_PI_2 + PI;

    let range = (WORLD_SIZE / BLUFF_SPRITE_SIZE).abs().round() as u32;
    let image = "terrain/bluff.png";

    for i in 1..range {
        let j = BLUFF_SPRITE_SIZE * i as f32 - WORLD_SIZE_HALF;
        spawn_sprite(world, Vec3::new(j, -n, z), r1, image);
        spawn_sprite(world, Vec3::new(j, n, z), r2, image);
        spawn_sprite(world, Vec3::new(-n, j, z), r3, image);
        spawn_sprite(world, Vec3::new(n, j, z), r4, image);
    }

    let image_corner = "terrain/bluff_corner.png";
    spawn_sprite(world, Vec3::new(-n, -n, z), r1, image_corner);
    spawn_sprite(world, Vec3::new(n, n, z), r2, image_corner);
    spawn_sprite(world, Vec3::new(-n, n, z), r3, image_corner);
    spawn_sprite(world, Vec3::new(n, -n, z), r4, image_corner);
}

fn spawn_trees(world: &mut World) {
    let mut rng = Pcg32::seed_from_u64(100);
    let trees = f32::max(0.0, TREES_QUANTITY) as usize;
    let image = [
        "terrain/tree_0.png",
        "terrain/tree_1.png",
        "terrain/tree_2.png",
    ];

    let range = WORLD_SIZE_VISUAL / 2.0;
    let mut occupied_positions = Vec::with_capacity(trees);

    for _ in 0..trees {
        for _ in 0..TREE_FIND_POSITION_ATTEMPTS {
            let position = Vec2::new(rng.gen_range(-range..range), rng.gen_range(-range..range));

            if is_position_free(position, &occupied_positions) {
                let texture = image.choose(&mut rng).unwrap_or(&image[0]);

                spawn_sprite(
                    world,
                    position.extend(LAYER_TREE),
                    rng.gen_range(0.0..TAU),
                    texture,
                );

                occupied_positions.push(position);
                break;
            }
        }
    }
}

fn spawn_sprite(world: &mut World, position: Vec3, direction: f32, path: &'static str) {
    let Some(texture) = world.resource::<AssetServer>().get_handle(path) else {
        log::warn!("Can't find texture: {}", path);
        return;
    };

    world.spawn(SpriteBundle {
        transform: Transform {
            translation: position,
            rotation: Quat::from_rotation_z(direction),
            scale: TRANSFORM_SCALE,
        },
        texture,
        ..Default::default()
    });
}

fn play_audio(audio: &AudioTracker) {
    audio.queue(AudioPlay {
        path: "sounds/ambience_music".into(),
        volume: 0.3,
        duration: Duration::MAX,
        ..AudioPlay::DEFAULT
    });

    audio.queue(AudioPlay {
        path: "sounds/ambience_nature".into(),
        volume: 0.3,
        duration: Duration::MAX,
        ..AudioPlay::DEFAULT
    });

    audio.queue(AudioPlay {
        path: "sounds/heartbeat".into(),
        duration: Duration::MAX,
        ..AudioPlay::DEFAULT
    });
}

fn is_position_free(position: Vec2, occupied_positions: &[Vec2]) -> bool {
    if is_position_on_bluff(position.x) || is_position_on_bluff(position.y) {
        return false;
    }

    return occupied_positions
        .iter()
        .all(|p| p.is_far(position, TREE_BUFFER_ZONE));
}

fn is_position_on_bluff(n: f32) -> bool {
    return (n.abs() - WORLD_SIZE_HALF).abs() < TREE_BUFFER_ZONE / 2.0;
}
