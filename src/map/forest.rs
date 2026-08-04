use crate::{
    data::{LAYER_GROUND, LAYER_TREE, WORLD_SIZE, WORLD_SIZE_HALF, WORLD_SIZE_VISUAL},
    map::Map,
    plugin::{AudioPlay, AudioTracker, TerrainSpawn, TileBlend},
    util::ext::{RngExt2, Vec2Ext},
};
use bevy::{
    color::{Color, Srgba},
    ecs::{system::Command, world::World},
    math::{Vec2, Vec3},
};
use rand::{seq::IndexedRandom, RngExt, SeedableRng};
use rand_pcg::Pcg32;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

const TREES_DENSITY: f32 = 0.025;
const TREE_BUFFER_ZONE: f32 = 3.2;
const TREE_FIND_POSITION_ATTEMPTS: usize = 32;
const BLUFF_SPRITE_SIZE: f32 = 4.0;

pub struct ForestMap;

impl Map for ForestMap {
    fn generate(&self, world: &mut World) {
        world.commands().queue(TerrainSpawn {
            image: "terrain/grass.png",
        });

        spawn_bluffs(world);
        spawn_trees(world);
        play_audio(world);
    }
}

fn spawn_bluffs(world: &mut World) {
    let n = WORLD_SIZE_HALF;
    let r1 = PI;
    let r2 = 0.0;
    let r3 = FRAC_PI_2;
    let r4 = FRAC_PI_2 + PI;

    let range = (WORLD_SIZE / BLUFF_SPRITE_SIZE).abs().round() as u32;
    let blend = |w: &mut World, i: &'static str, x: f32, y: f32, r: f32| {
        TileBlend::Image {
            image: i,
            color: Color::default(),
            position: Vec3::new(x, y, LAYER_GROUND),
            direction: r,
            size: BLUFF_SPRITE_SIZE,
            flip: false,
        }
        .apply(w);
    };

    let mut image = "terrain/bluff.png";
    for i in 1..range {
        let j = BLUFF_SPRITE_SIZE * i as f32 - WORLD_SIZE_HALF;
        blend(world, image, j, -n, r1);
        blend(world, image, j, n, r2);
        blend(world, image, -n, j, r3);
        blend(world, image, n, j, r4);
    }

    image = "terrain/bluff_corner.png";
    blend(world, image, -n, -n, r1);
    blend(world, image, n, n, r2);
    blend(world, image, -n, n, r3);
    blend(world, image, n, -n, r4);
}

fn spawn_trees(world: &mut World) {
    let mut rng = Pcg32::seed_from_u64(250);

    let trees = usize::max(
        0,
        (WORLD_SIZE_VISUAL * WORLD_SIZE_VISUAL * TREES_DENSITY) as usize,
    );

    let images = [
        (1.0, "terrain/tree_1.png", 3.0, 4.0),
        (1.0, "terrain/tree_2.png", 3.0, 4.0),
        (0.5, "terrain/tree_spruce.png", 1.5, 2.5),
    ];

    let range = WORLD_SIZE_VISUAL / 2.0;
    let mut occupied_positions = Vec::with_capacity(trees);

    for _ in 0..trees {
        for _ in 0..TREE_FIND_POSITION_ATTEMPTS {
            let position = Vec2::new(
                rng.random_range(-range..range),
                rng.random_range(-range..range),
            );

            if is_position_free(position, &occupied_positions) {
                let (_weight, image, size_min, size_max) = images
                    .choose_weighted(&mut rng, |i| i.0)
                    .unwrap_or(&images[0]);

                let color_fuzz = 0.06;
                let color = Srgba::new(
                    1.0 - rng.random_range(0.0..color_fuzz),
                    1.0 - rng.random_range(0.0..color_fuzz),
                    1.0 - rng.random_range(0.0..color_fuzz),
                    0.95,
                );

                TileBlend::Image {
                    image,
                    color: color.into(),
                    position: position.extend(LAYER_TREE),
                    direction: rng.random_range(0.0..TAU),
                    size: rng.gen_range_safely(*size_min, *size_max),
                    flip: rng.random(),
                }
                .apply(world);

                occupied_positions.push(position);
                break;
            }
        }
    }

    log::debug!("Spawned trees: {}", occupied_positions.len());
}

fn play_audio(world: &mut World) {
    let audio = world.resource::<AudioTracker>();

    audio.queue(AudioPlay {
        path: "sounds/ambience_music".into(),
        volume: 0.3,
        duration: AudioPlay::DURATION_FOREVER,
        ..AudioPlay::DEFAULT
    });

    audio.queue(AudioPlay {
        path: "sounds/ambience_nature".into(),
        volume: 0.2,
        duration: AudioPlay::DURATION_FOREVER,
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
