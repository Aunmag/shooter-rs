use crate::data::LAYER_BLUFF;
use crate::data::LAYER_TREE;
use crate::data::WORLD_SIZE;
use crate::data::WORLD_SIZE_HALF;
use crate::data::WORLD_SIZE_VISUAL;
use crate::resources::Sprite;
use crate::resources::SpriteResource;
use crate::utils::math::are_closer_than;
use crate::utils::WorldExtCustom;
use amethyst::ecs::Entity;
use amethyst::ecs::World;
use amethyst::ecs::WorldExt;
use amethyst::renderer::SpriteRender;
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

pub fn create_decorations(world: &mut World, root: Entity) {
    create_trees(world, root);
    create_bluffs(world, root);
}

fn create_trees(world: &mut World, root: Entity) {
    let mut randomizer = Pcg32::seed_from_u64(100);
    let trees_quantity;

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    if TREES_QUANTITY >= 0.0 {
        trees_quantity = TREES_QUANTITY.abs().round() as usize;
    } else {
        trees_quantity = 0;
    }

    let sprite_0;
    let sprite_1;
    let sprite_2;

    {
        let sprites = world.read_resource::<SpriteResource>();
        sprite_0 = sprites.get(Sprite::Tree0).map(|s| SpriteRender::new(s, 0));
        sprite_1 = sprites.get(Sprite::Tree1).map(|s| SpriteRender::new(s, 0));
        sprite_2 = sprites.get(Sprite::Tree2).map(|s| SpriteRender::new(s, 0));
    }

    let range = WORLD_SIZE_VISUAL / 2.0;
    let mut occupied_positions = Vec::with_capacity(trees_quantity);

    for _ in 0..trees_quantity {
        for _ in 0..TREE_FIND_POSITION_ATTEMPTS {
            let x = randomizer.gen_range(-range..range);
            let y = randomizer.gen_range(-range..range);

            if is_position_free(x, y, &occupied_positions) {
                let sprite = match randomizer.gen_range(0..=2) {
                    0 => &sprite_0,
                    1 => &sprite_1,
                    2 => &sprite_2,
                    _ => &None,
                };

                if let Some(sprite) = sprite.as_ref() {
                    world.create_simple_sprite(
                        root,
                        x,
                        y,
                        LAYER_TREE,
                        randomizer.gen_range(0.0..TAU),
                        sprite.clone(),
                    );
                }

                occupied_positions.push((x, y));
                break;
            }
        }
    }
}

// TODO: Maybe render bluff corner as tile map
fn create_bluffs(world: &mut World, root: Entity) {
    let n = WORLD_SIZE_HALF;
    let z = LAYER_BLUFF;
    let r1 = PI;
    let r2 = 0.0;
    let r3 = FRAC_PI_2;
    let r4 = FRAC_PI_2 + PI;

    let sprite_flat;
    let sprite_corner;

    {
        let sprites = world.read_resource::<SpriteResource>();

        sprite_flat = sprites
            .get(Sprite::Bluff)
            .map(|s| SpriteRender::new(s, 0));

        sprite_corner = sprites
            .get(Sprite::BluffCorner)
            .map(|s| SpriteRender::new(s, 0));
    }

    if let Some(sprite) = sprite_flat {
        let range;

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        {
            range = (WORLD_SIZE / BLUFF_SPRITE_SIZE).abs().round() as u32;
        }

        for i in 1..range {
            let j = BLUFF_SPRITE_SIZE * i as f32 - WORLD_SIZE_HALF;
            world.create_simple_sprite(root, j, -n, z, r1, sprite.clone());
            world.create_simple_sprite(root, j, n, z, r2, sprite.clone());
            world.create_simple_sprite(root, -n, j, z, r3, sprite.clone());
            world.create_simple_sprite(root, n, j, z, r4, sprite.clone());
        }
    }

    if let Some(sprite) = sprite_corner {
        world.create_simple_sprite(root, -n, -n, z, r1, sprite.clone());
        world.create_simple_sprite(root, n, n, z, r2, sprite.clone());
        world.create_simple_sprite(root, -n, n, z, r3, sprite.clone());
        world.create_simple_sprite(root, n, -n, z, r4, sprite);
    }
}

fn is_position_free(x: f32, y: f32, occupied_positions: &[(f32, f32)]) -> bool {
    if is_on_bluff(x) || is_on_bluff(y) {
        return false;
    }

    for &(occupied_x, occupied_y) in occupied_positions {
        if are_closer_than(x, y, occupied_x, occupied_y, TREE_BUFFER_ZONE) {
            return false;
        }
    }

    return true;
}

fn is_on_bluff(n: f32) -> bool {
    return (n.abs() - WORLD_SIZE_HALF).abs() < TREE_BUFFER_ZONE / 2.0;
}
