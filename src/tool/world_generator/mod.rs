mod biome;

use crate::{
    data::{
        LAYER_GROUND, LAYER_TREE, PIXELS_PER_METER, VIEW_DISTANCE, WORLD_SIZE, WORLD_SIZE_HALF,
    },
    plugin::TileBlend,
    util::ext::Vec2Ext,
};
use bevy::{
    asset::AssetServer,
    ecs::{system::Command, world::World},
    math::{Vec2, Vec3},
};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

use self::biome::Biome;

// TODO: draw fallen trees
// TODO: draw trees
// TODO: tweak density
// TODO: fix stumps color, match it from fallen tree
// TODO: resize all images
// TODO: check larges image size
// TODO: rework bluff

// TODO: dynamic density by noise

const WORLD_SIZE_VISUAL: f32 = WORLD_SIZE + VIEW_DISTANCE;

const DECOR_PER_METER: f32 = 0.02;
const DECOR_QUANTITY: u32 = (WORLD_SIZE_VISUAL * WORLD_SIZE_VISUAL * DECOR_PER_METER) as u32;

const TREES_PER_METER: f32 = 0.02; // TODO: tweak
const TREES_QUANTITY: u32 = (WORLD_SIZE_VISUAL * WORLD_SIZE_VISUAL * TREES_PER_METER) as u32;

const FIND_POSITION_ATTEMPTS: u8 = 255; // TODO: teak
const BLUFF_SPRITE_SIZE: f32 = 4.0;

// TODO: move to tool?
pub struct WorldGenerator<'a> {
    world: &'a mut World,
    rng: Pcg32,
    occupied_bottom: Vec<(Vec2, f32)>,
    occupied_top: Vec<(Vec2, f32)>,

    // biome_combat: Biome,
    // biome_camp: Biome,
    // biome_rocky: Biome,
    // biome_forest: Biome,
    // biome_swamp: Biome,
    // biome_dry: Biome,
}

impl<'a> WorldGenerator<'a> {
    pub fn new(world: &'a mut World, seed: u64) -> WorldGenerator<'a> {
        return Self {
            world,
            rng: Pcg32::seed_from_u64(seed),
            occupied_bottom: Vec::new(),
            occupied_top: Vec::new(),
        };
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
            self.blend_sprite(Vec3::new(j, -n, z), r1, image, false, false, None);
            self.blend_sprite(Vec3::new(j, n, z), r2, image, false, false, None);
            self.blend_sprite(Vec3::new(-n, j, z), r3, image, false, false, None);
            self.blend_sprite(Vec3::new(n, j, z), r4, image, false, false, None);
        }

        let image_corner = "decor/bluff_corner.png";
        self.blend_sprite(Vec3::new(-n, -n, z), r1, image_corner, false, false, None);
        self.blend_sprite(Vec3::new(n, n, z), r2, image_corner, false, false, None);
        self.blend_sprite(Vec3::new(-n, n, z), r3, image_corner, false, false, None);
        self.blend_sprite(Vec3::new(n, -n, z), r4, image_corner, false, false, None);
    }

    fn spawn_ground_decor(&mut self) {
        log::debug!("Spawning {} decorations...", DECOR_QUANTITY);

        let groups = [
            ImageGroup(4.0, vec![Image::new("decor/crater.png", 48, 6.0)]),
            ImageGroup(
                8.0,
                vec![
                    Image::new("decor/puddle_0.png", 56, 2.5),
                    Image::new("decor/puddle_1.png", 56, 2.5),
                    Image::new("decor/puddle_2.png", 56, 2.5),
                ],
            ),
            ImageGroup(
                1.0,
                vec![
                    Image::new("decor/campfire_0.png", 32, 1.0),
                    Image::new("decor/campfire_1.png", 32, 1.0),
                ],
            ),
            ImageGroup(
                3.0,
                vec![
                    Image::new("decor/stump_0.png", 48, 1.5),
                    Image::new("decor/stump_1.png", 40, 1.2),
                ],
            ),
            ImageGroup(
                1.0,
                vec![
                    Image::new("decor/dead_0.png", 50, 1.0),
                    Image::new("decor/dead_1.png", 50, 1.0),
                    Image::new("decor/dead_2.png", 50, 1.0),
                    Image::new("decor/dead_3.png", 50, 1.0),
                    Image::new("decor/dead_4.png", 50, 1.0),
                ],
            ),
            ImageGroup(
                4.0,
                vec![
                    Image::new("decor/rock_0.png", 32, 2.0),
                    Image::new("decor/rock_1.png", 32, 2.0),
                    Image::new("decor/rock_2.png", 32, 2.0),
                    Image::new("decor/rock_3.png", 32, 2.0),
                ],
            ),
            ImageGroup(
                0.5,
                vec![
                    Image::new("decor/tent_0.png", 50, 1.2),
                    Image::new("decor/tent_1.png", 50, 1.2),
                    Image::new("decor/tent_2.png", 50, 2.0),
                    Image::new("decor/tent_3.png", 50, 2.0),
                    Image::new("decor/tent_4.png", 50, 1.2),
                ],
            ),
            ImageGroup(8.0, vec![Image::new("decor/branch_0.png", 64, 2.5)]),
        ];

        self.spawn_decor_layer(LAYER_GROUND, DECOR_QUANTITY, &groups);
    }

    fn spawn_trees(&mut self) {
        log::debug!("Spawning {} trees...", TREES_QUANTITY);
        let groups = [ImageGroup(
            1.0,
            vec![Image::new("decor/tree.png", 96, 1.33)],
        )];
        self.spawn_decor_layer(LAYER_TREE, TREES_QUANTITY, &groups);
    }

    fn spawn_decor_layer(&mut self, layer: f32, count: u32, groups: &[ImageGroup]) {
        for _ in 0..count {
            let Ok(group) = groups.choose_weighted(&mut self.rng, |i| i.0) else {
                // TODO: panic on debug?
                break;
            };

            let Some(image) = group.1.choose(&mut self.rng) else {
                // TODO: panic on debug?
                continue;
            };

            let size = image.gen_size(&mut self.rng);
            let radius = size / 2.0 / PIXELS_PER_METER;
            let radius_bottom;
            let radius_top;

            if layer == LAYER_TREE {
                radius_bottom = radius / 4.0;
                radius_top = radius;
            } else {
                radius_bottom = radius;
                radius_top = 0.0;
            }

            let Some(position) = self.find_position(radius_bottom, radius_top) else {
                continue;
            };

            let direction = self.rng.gen_range(0.0..TAU);
            let flip_x = self.rng.gen();
            let flip_y = self.rng.gen();

            self.blend_sprite(
                position.extend(layer),
                direction,
                image.path,
                flip_x,
                flip_y,
                Some(size),
            );
        }
    }

    fn blend_sprite(
        &mut self,
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

    fn find_position(&mut self, radius_bottom: f32, radius_top: f32) -> Option<Vec2> {
        // TODO: check bluffs
        let range = WORLD_SIZE_VISUAL / 2.0;

        for _ in 0..FIND_POSITION_ATTEMPTS {
            let position = Vec2::new(
                self.rng.gen_range(-range..range),
                self.rng.gen_range(-range..range),
            );

            if radius_bottom > 0.0 && find_collision(&self.occupied_bottom, position, radius_bottom)
            {
                continue;
            }

            if radius_top > 0.0 && find_collision(&self.occupied_top, position, radius_top) {
                continue;
            }

            if radius_bottom > 0.0 {
                self.occupied_bottom.push((position, radius_bottom));
            }

            if radius_top > 0.0 {
                self.occupied_top.push((position, radius_top));
            }

            return Some(position);
        }

        return None;
    }
}

fn find_collision(obstacles: &[(Vec2, f32)], position: Vec2, radius: f32) -> bool {
    for (obstacle, obstacle_radius) in obstacles {
        if position.is_close(*obstacle, radius + *obstacle_radius) {
            return true;
        }
    }

    return false;
}

struct ImageGroup(f32, Vec<Image>);

struct Image {
    path: &'static str,
    size_min: u8,
    size_mul: f32,
}

impl Image {
    const fn new(path: &'static str, size_min: u8, size_mul: f32) -> Self {
        return Self {
            path,
            size_min,
            size_mul,
        };
    }

    fn gen_size(&self, rng: &mut Pcg32) -> f32 {
        let size_min = self.size_min as f32;
        let size_max = size_min * f32::max(self.size_mul, 1.1);
        return rng.gen_range(size_min..size_max); // TODO: lower change for big size
    }
}
