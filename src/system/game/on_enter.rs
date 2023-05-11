use crate::command::ActorBotSet;
use crate::command::ActorPlayerSet;
use crate::command::ActorSet;
use crate::command::CursorLock;
use crate::command::TerrainInit;
use crate::component::ActorConfig;
use crate::data::LAYER_BLUFF;
use crate::data::LAYER_TREE;
use crate::data::WORLD_SIZE;
use crate::data::WORLD_SIZE_HALF;
use crate::data::WORLD_SIZE_VISUAL;
use crate::model::Position;
use crate::resource::GameType;
use crate::util::ext::Vec2Ext;
use bevy::asset::AssetServer;
use bevy::asset::Handle;
use bevy::math::Vec2;
use bevy::prelude::Camera2dBundle;
use bevy::prelude::Commands;
use bevy::prelude::Image;
use bevy::prelude::Res;
use bevy::prelude::SpriteBundle;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use std::f32::consts::TAU;

const TREES_PER_METER: f32 = 0.02;
const TREES_QUANTITY: f32 = WORLD_SIZE_VISUAL * WORLD_SIZE_VISUAL * TREES_PER_METER;
const TREE_BUFFER_ZONE: f32 = 3.2;
const TREE_FIND_POSITION_ATTEMPTS: usize = 32;
const BLUFF_SPRITE_SIZE: f32 = 4.0;

pub fn on_enter(mut commands: Commands, assets: Res<AssetServer>, game_type: Res<GameType>) {
    commands.add(CursorLock(true));
    commands.add(TerrainInit);
    commands.spawn_bundle(Camera2dBundle::default());

    if game_type.is_server() {
        spawn_player(&mut commands);

        for i in 0..2 {
            spawn_zombie(&mut commands, 5.0 * (0.5 - i as f32));
        }
    }

    spawn_bluffs(&mut commands, &assets);
    spawn_trees(&mut commands, &assets);
}

fn spawn_player(commands: &mut Commands) {
    let entity = commands.spawn().id();

    commands.add(ActorSet {
        entity,
        config: ActorConfig::HUMAN,
        position: Position::default(),
        is_ghost: false,
    });

    commands.add(ActorPlayerSet(entity));
}

fn spawn_zombie(commands: &mut Commands, position_x: f32) {
    let entity = commands.spawn().id();

    commands.add(ActorSet {
        entity,
        config: ActorConfig::ZOMBIE,
        position: Position::new(position_x, 0.0, 0.0),
        is_ghost: false,
    });

    commands.add(ActorBotSet(entity));
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
    let mut randomizer = Pcg32::seed_from_u64(100);
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
            let position = Vec2::new(
                randomizer.gen_range(-range..range),
                randomizer.gen_range(-range..range),
            );

            if is_position_free(position, &occupied_positions) {
                let texture = textures
                    .choose(&mut randomizer)
                    .unwrap_or(&textures[0])
                    .clone();

                spawn_sprite(
                    commands,
                    position.x,
                    position.y,
                    LAYER_TREE,
                    randomizer.gen_range(0.0..TAU),
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
    commands.spawn_bundle(SpriteBundle {
        transform: Position::new(x, y, direction).as_transform(z),
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
