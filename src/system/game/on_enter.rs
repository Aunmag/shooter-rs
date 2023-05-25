use crate::{
    command::{AudioPlay, CursorGrab, TerrainInit},
    data::{LAYER_BLUFF, LAYER_TREE, WORLD_SIZE, WORLD_SIZE_HALF, WORLD_SIZE_VISUAL},
    model::TransformLite,
    util::ext::Vec2Ext,
};
use bevy::{
    asset::{AssetServer, Handle},
    math::Vec2,
    prelude::{Camera2dBundle, Commands, Image, Res, SpriteBundle},
};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

const TREES_PER_METER: f32 = 0.02;
const TREES_QUANTITY: f32 = WORLD_SIZE_VISUAL * WORLD_SIZE_VISUAL * TREES_PER_METER;
const TREE_BUFFER_ZONE: f32 = 3.2;
const TREE_FIND_POSITION_ATTEMPTS: usize = 32;
const BLUFF_SPRITE_SIZE: f32 = 4.0;

pub fn on_enter(mut commands: Commands, assets: Res<AssetServer>) {
    commands.add(CursorGrab(true));
    commands.add(TerrainInit);
    commands.spawn(Camera2dBundle::default());
    spawn_bluffs(&mut commands, &assets);
    spawn_trees(&mut commands, &assets);

    commands.add(AudioPlay {
        path: "sounds/ambience_music.ogg",
        volume: 0.3,
        repeat: true,
        ..AudioPlay::DEFAULT
    });

    commands.add(AudioPlay {
        path: "sounds/ambience_nature.ogg",
        volume: 0.3,
        repeat: true,
        ..AudioPlay::DEFAULT
    });
}

// TODO: maybe render bluff corner as tile map
fn spawn_bluffs(commands: &mut Commands, assets: &AssetServer) {
    let n = WORLD_SIZE_HALF;
    let z = LAYER_BLUFF;
    let r1 = PI;
    let r2 = 0.0;
    let r3 = FRAC_PI_2;
    let r4 = FRAC_PI_2 + PI;

    let range = (WORLD_SIZE / BLUFF_SPRITE_SIZE).abs().round() as u32;
    let texture = assets.get_handle("terrain/bluff.png");

    for i in 1..range {
        let j = BLUFF_SPRITE_SIZE * i as f32 - WORLD_SIZE_HALF;
        spawn_sprite(commands, j, -n, z, r1, texture.clone());
        spawn_sprite(commands, j, n, z, r2, texture.clone());
        spawn_sprite(commands, -n, j, z, r3, texture.clone());
        spawn_sprite(commands, n, j, z, r4, texture.clone());
    }

    let texture_corner = assets.get_handle("terrain/bluff_corner.png");
    spawn_sprite(commands, -n, -n, z, r1, texture_corner.clone());
    spawn_sprite(commands, n, n, z, r2, texture_corner.clone());
    spawn_sprite(commands, -n, n, z, r3, texture_corner.clone());
    spawn_sprite(commands, n, -n, z, r4, texture_corner);
}

fn spawn_trees(commands: &mut Commands, assets: &AssetServer) {
    let mut rng = Pcg32::seed_from_u64(100);
    let trees_quantity = f32::max(0.0, TREES_QUANTITY) as usize;

    let textures = [
        assets.get_handle("terrain/tree_0.png"),
        assets.get_handle("terrain/tree_1.png"),
        assets.get_handle("terrain/tree_2.png"),
    ];

    let range = WORLD_SIZE_VISUAL / 2.0;
    let mut occupied_positions = Vec::with_capacity(trees_quantity);

    for _ in 0..trees_quantity {
        for _ in 0..TREE_FIND_POSITION_ATTEMPTS {
            let position = Vec2::new(rng.gen_range(-range..range), rng.gen_range(-range..range));

            if is_position_free(position, &occupied_positions) {
                let texture = textures.choose(&mut rng).unwrap_or(&textures[0]).clone();

                spawn_sprite(
                    commands,
                    position.x,
                    position.y,
                    LAYER_TREE,
                    rng.gen_range(0.0..TAU),
                    texture,
                );

                occupied_positions.push(position);
                break;
            }
        }
    }
}

fn spawn_sprite(
    commands: &mut Commands,
    x: f32,
    y: f32,
    z: f32,
    direction: f32,
    texture: Handle<Image>,
) {
    commands.spawn(SpriteBundle {
        transform: TransformLite::new(x, y, direction).as_transform(z),
        texture,
        ..Default::default()
    });
}

fn is_position_free(position: Vec2, occupied_positions: &[Vec2]) -> bool {
    if is_position_on_bluff(position.x) || is_position_on_bluff(position.y) {
        return false;
    }

    return occupied_positions
        .iter()
        .all(|p| (*p - position).is_longer_than(TREE_BUFFER_ZONE));
}

fn is_position_on_bluff(n: f32) -> bool {
    return (n.abs() - WORLD_SIZE_HALF).abs() < TREE_BUFFER_ZONE / 2.0; // TODO: why two abs?
}
