use bevy::app::{App, Plugin};
use rand::SeedableRng;
use rand_pcg::Pcg32;
use crate::{model::AppState, util::ext::AppExt};
use crate::{
    command::CursorGrab,
    data::{LAYER_GROUND, LAYER_TREE, WORLD_SIZE, WORLD_SIZE_HALF, WORLD_SIZE_VISUAL},
    model::AudioPlay,
    plugin::TileBlend,
    resource::AudioTracker,
    util::ext::Vec2Ext,
};
use bevy::{
    asset::AssetServer,
    ecs::{system::Command, world::World},
    math::{Vec2, Vec3},
    prelude::Camera2dBundle,
};
use rand::{seq::SliceRandom, Rng};
use std::f32::consts::{FRAC_PI_2, PI, TAU};

// TODO: draw fallen trees
// TODO: draw trees
// TODO: tweak density
// TODO: fix stumps color, match it from fallen tree
// TODO: resize all images
// TODO: check larges image size
// TODO: rework bluff

const TREES_PER_METER: f32 = 0.02;
const TREES_QUANTITY: f32 = WORLD_SIZE_VISUAL * WORLD_SIZE_VISUAL * TREES_PER_METER;
const TREE_BUFFER_ZONE: f32 = 3.2; // TODO: make dynamic
const TREE_FIND_POSITION_ATTEMPTS: usize = 32;
const BLUFF_SPRITE_SIZE: f32 = 4.0;

// TODO: move to tool?
pub struct WorldGenerator<'a> {
    world: &mut World,
    rng: Pcg32,
    occupied_positions: Vec,
}

impl<'a> WorldGenerator<'a> {
    // TODO: get seed from config
    pub fn new<'a>(world: &'a mut World, seed: u64) -> WorldGenerator<'a> {
        return Self { world, rng: Pcg32::from_seed(seed), occupied_positions: Vec::new() };
    }

    pub fn generate(&mut self) {
        self.spawn_bluffs();
        self.spawn_ground_decor();
        self.spawn_trees();
    }

    fn spawn_bluffs(&mut self) {
        let n = WORLD_SIZE_HALF;
        let z = LAYER_GROUND;

        let r1 = PI;
        let r2 = 0.0;
        let r3 = FRAC_PI_2;
        let r4 = FRAC_PI_2 + PI;

        let range = (WORLD_SIZE / BLUFF_SPRITE_SIZE).abs().round() as u32;
        let image = "decor/bluff.png";

        for i in 1..range {
            let j = BLUFF_SPRITE_SIZE * i as f32 - WORLD_SIZE_HALF;
            blend_sprite(world, Vec3::new(j, -n, z), r1, image, false, false, None);
            blend_sprite(world, Vec3::new(j, n, z), r2, image, false, false, None);
            blend_sprite(world, Vec3::new(-n, j, z), r3, image, false, false, None);
            blend_sprite(world, Vec3::new(n, j, z), r4, image, false, false, None);
        }

        let image_corner = "decor/bluff_corner.png";
        blend_sprite(world, Vec3::new(-n, -n, z), r1, image_corner, false, false, None);
        blend_sprite(world, Vec3::new(n, n, z), r2, image_corner, false, false, None);
        blend_sprite(world, Vec3::new(-n, n, z), r3, image_corner, false, false, None);
        blend_sprite(world, Vec3::new(n, -n, z), r4, image_corner, false, false, None);
    }

    fn spawn_ground_decor(&mut self) {
        self.spawn_decor_layer(
            LAYER_GROUND,
            &[
                (4.0, "decor/crater.png", 48, 6.0),
                // (8.0 / 3.0, "decor/puddle_0.png", 56, 2.5),
                // (8.0 / 3.0, "decor/puddle_1.png", 56, 2.5),
                // (8.0 / 3.0, "decor/puddle_2.png", 56, 2.5),
                (1.0 / 2.0, "decor/campfire_0.png", 32, 1.0),
                (1.0 / 2.0, "decor/campfire_1.png", 32, 1.0),
                // (3.0 / 2.0, "decor/stump_0.png", 48, 1.5),
                // (3.0 / 2.0, "decor/stump_1.png", 40, 1.2),
                (1.0 / 5.0, "decor/dead_0.png", 50, 1.0),
                (1.0 / 5.0, "decor/dead_1.png", 50, 1.0),
                (1.0 / 5.0, "decor/dead_2.png", 50, 1.0),
                (1.0 / 5.0, "decor/dead_3.png", 50, 1.0),
                (1.0 / 5.0, "decor/dead_4.png", 50, 1.0),
                // (4.0 / 4.0, "decor/rock_0.png", 32, 2.0),
                // (4.0 / 4.0, "decor/rock_1.png", 32, 2.0),
                // (4.0 / 4.0, "decor/rock_2.png", 32, 2.0),
                // (4.0 / 4.0, "decor/rock_3.png", 32, 2.0),
                (0.5 / 5.0, "decor/tent_0.png", 50, 1.2),
                (0.5 / 5.0, "decor/tent_1.png", 50, 1.2),
                (0.5 / 5.0, "decor/tent_2.png", 50, 2.0),
                (0.5 / 5.0, "decor/tent_3.png", 50, 2.0),
                (0.5 / 5.0, "decor/tent_4.png", 50, 1.2),
                // (8.0, "decor/branch_0.png", 64, 2.5),
            ],
        );
    }

    fn spawn_trees(&mut self) {
        self.spawn_decor_layer(
            LAYER_TREE,
            &[
                (1.0, "decor/tree.png", 96, 1.33),
            ],
        );
    }

    // TODO: rework
    fn spawn_decor_layer(
        &mut self,
        layer: f32,
        images: &[(f32, &'static str, u8, f32)],
    ) {
        let trees = f32::max(0.0, TREES_QUANTITY) as usize;
        let range = WORLD_SIZE_VISUAL / 2.0;

        for _ in 0..trees {
            for _ in 0..TREE_FIND_POSITION_ATTEMPTS {
                let position = Vec2::new(rng.gen_range(-range..range), rng.gen_range(-range..range));

                if self.is_position_free(position) {
                    let (_, texture, size_min, size_mul) =
                        images.choose_weighted(rng, |i| i.0).unwrap_or(&images[0]);

                    let size_min = *size_min as f32;
                    let size_max = size_min * f32::max(*size_mul, 1.1); // TODO: to const
                    let size = rng.gen_range(size_min..size_max); // TODO: more change for lower value

                    self.blend_sprite(
                        position.extend(layer),
                        rng.gen_range(0.0..TAU),
                        texture,
                        rng.gen(),
                        rng.gen(),
                        Some(size),
                    );

                    break;
                }
            }
        }
    }

    fn blend_sprite(
        &self,
        position: Vec3,
        direction: f32,
        path: &'static str,
        flip_x: bool,
        flip_y: bool,
        resize: Option<f32>,
    ) {
        let Some(image) = self.world.resource::<AssetServer>().get_handle(path) else {
            log::warn!("Image {} not found", path);
            return;
        };

        TileBlend::Image {
            image,
            position,
            direction,
            flip_x,
            flip_y,
            resize,
        }
        .apply(self.world);
    }

    fn is_position_free(&self, position: Vec2) -> bool {
        if Self::is_position_on_bluff(position.x) || Self::is_position_on_bluff(position.y) {
            return false;
        }

        return occupied_positions
            .iter()
            .all(|p| p.is_far(position, TREE_BUFFER_ZONE));
    }

    fn is_position_on_bluff(n: f32) -> bool {
        return (n.abs() - WORLD_SIZE_HALF).abs() < TREE_BUFFER_ZONE / 2.0;
    }
}
