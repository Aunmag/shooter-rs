mod noise_generator;
mod plot;
mod util;

use core::num;

use anyhow::Result;
use image::{imageops::FilterType, DynamicImage, GenericImage, GenericImageView, GrayImage, ImageBuffer, Rgba, RgbaImage};
use noise_generator::NoiseGenerator;
use plot::Histogram;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

const WORLD_SIZE: u32 = 1024;

const TERRAIN_1: &str = "C:/Users/Aunmag/YandexDisk/Shooter/assets_raw/images/terrain/grass_1.png";
const TERRAIN_2: &str = "C:/Users/Aunmag/YandexDisk/Shooter/assets_raw/images/terrain/grass_980120234.png";
const TERRAIN_3: &str = "C:/Users/Aunmag/YandexDisk/Shooter/assets_raw/images/terrain/sand_grass_1.png";

const TERRAIN_SIZE: u32 = 256;
const SEED: u64 = 23511;

type Img = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn main() -> Result<()> {
    // if true {
    //     gen_inclusion_mask();
    //     return Ok(());
    // }

    let mut world = RgbaImage::new(WORLD_SIZE, WORLD_SIZE);

    let mut t1 = load_terrain(TERRAIN_1)?;
    let mut t2 = load_terrain(TERRAIN_2)?;
    let mut t3 = load_terrain(TERRAIN_3)?;

    // img.copy_from(other, x, y)
    let n = WORLD_SIZE / TERRAIN_SIZE;
    for x in 0..n {
        for y in 0..n {
            let x = TERRAIN_SIZE * x;
            let y = TERRAIN_SIZE * y;

            // world.copy_from(&t1, TERRAIN_SIZE * x, TERRAIN_SIZE * y)?;

            world.copy_from(&t1, x, y)?;

            mask_image(&mut t2, x, y, 194_222);
            image::imageops::overlay(&mut world, &t2, x as i64, y as i64);

            mask_image(&mut t3, x, y, 413_331);
            image::imageops::overlay(&mut world, &t3, x as i64, y as i64);
        }
    }

    world.save("world.png")?;

    return Ok(());
}

fn load_terrain(path: &str) -> Result<Img> {
    let mut terrain = image::open(path)?
        .resize_exact(TERRAIN_SIZE, TERRAIN_SIZE, FilterType::Lanczos3)  // TODO: try other filter
        .into_rgba8();

    // compress_colors(&mut terrain);

    return Ok(terrain);
}

fn mask_image(image: &mut Img, x_offset: u32, y_offset: u32, seed: u64) {
    // TODO: tweak
    let threshold_max = 0.55;
    // let threshold_min = 0.6;
    let threshold_min = 0.53;

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let noise_generator = NoiseGenerator {
            seed: SEED,
            size: 1,
        };

        let noise = noise_generator.gen_smooth_with_octaves(
            (x + x_offset) as f32,
            (y + y_offset) as f32
        );

        if noise > threshold_max {
            pixel[3] = u8::MAX;
        } else if noise > threshold_min {
            let smooth = (noise - threshold_min) / (threshold_max - threshold_min); // TODO: simplify
            pixel[3] = (f32::from(u8::MAX) * smooth) as u8;
        } else {
            pixel[3] = 0;
        }
    }
}

fn gen_inclusion_mask() {
    let mut image = GrayImage::new(TERRAIN_SIZE, TERRAIN_SIZE);

    // TODO: tweak
    let threshold_max = 0.6;
    let threshold_min = 0.52;

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let noise_generator = NoiseGenerator {
            seed: SEED,
            size: 1,
        };

        let noise = noise_generator.gen_smooth_with_octaves(x as f32, y as f32);

        if noise > threshold_max {
            pixel[0] = u8::MAX;
        } else if noise > threshold_min {
            let smooth = (noise - threshold_min) / (threshold_max - threshold_min); // TODO: simplify
            pixel[0] = (f32::from(u8::MAX) * smooth) as u8;
        }

        // pixel[0] = (noise * 255.0) as u8;
    }

    image.save("noise.png").unwrap();
}

fn compress_colors(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for pixel in image.pixels_mut() {
        pixel[0] = compress_color(pixel[0]);
        pixel[1] = compress_color(pixel[1]);
        pixel[2] = compress_color(pixel[2]);
        // TODO: compress alpha?
    }
}

fn compress_color(color: u8) -> u8 {
    let steps = 16.0; // 10 - too small 16 seems ok
    return ((color as f64 / 255.0 * steps).round() / steps * 255.0) as u8;
}

enum Biome {
    Green,
    Desert,
    // Snow
}

struct BiomeModifier {
    terrain: String,
    terrain_extra: Vec<String>,
}
