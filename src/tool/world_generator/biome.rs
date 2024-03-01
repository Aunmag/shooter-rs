use std::f32::consts::PI;

use rand::Rng;
use rand_distr::num_traits::Saturating;
use rand_pcg::Pcg32;
use rand::SeedableRng;

use crate::util::{ext::RngExt, math::floor_by};

const OCTAVES: i32 = 3; // TODO: reduce?
const ROUGHNESS: f32 = 0.3;

pub struct Biome {
    seed: u64,
}

impl Biome {
    pub fn new(seed: u64) -> Self {
        return Self {
            seed,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> f32 {
        let mut n = 0.0;
        n += self.get_with_scaled_smoothed(x, y, 16);
        // n += self.get_with_scaled_smoothed(x, y, 1);
        // n /= 2.0;

        n *= n;
        // n = n.clamp(0.0, 1.0);

        let mut closest = (f32::INFINITY, n);
        for c in [0.0, 0.5, 1.0] {
            let d = (n - c).abs();
            if d < closest.0 {
                closest = (d, c);
            }
        }

        n = closest.1;

        return n;
    }

    // TODO: can I do scale later?
    pub fn get_scaled(&self, x: i32, y: i32, scale: i32) -> f32 {
        let seed = self.seed
            .wrapping_add_signed(i64::from(x / scale * scale).wrapping_mul(12345))
            .wrapping_add_signed(i64::from(y / scale * scale).wrapping_mul(54321))
            ;

        return Pcg32::seed_from_u64(seed).gen();
    }

    // TODO: can I do scale later?
    pub fn get_with_scaled_smoothed(&self, x: i32, y: i32, s: i32) -> f32 {
        let mut n = 0.0;
        // TODO: wrap add

        // center
        n += self.get_scaled(x, y, s) / 4.0;

        // sides
        n += self.get_scaled(x - s, y, s) / 8.0;
        n += self.get_scaled(x + s, y, s) / 8.0;
        n += self.get_scaled(x, y - s, s) / 8.0;
        n += self.get_scaled(x, y + s, s) / 8.0;

        // corners
        n += self.get_scaled(x - s, y - s, s) / 16.0;
        n += self.get_scaled(x + s, y - s, s) / 16.0;
        n += self.get_scaled(x - s, y + s, s) / 16.0;
        n += self.get_scaled(x + s, y + s, s) / 16.0;

        return n;
    }
}

#[cfg(test)]
mod tests {
    use image::{ImageBuffer, Rgb, RgbImage};
    use super::*;

    #[test]
    fn test_generate_seed_100() {
        let size = 1000;
        let biome = Biome::new(100);
        let mut image: RgbImage = ImageBuffer::new(size, size);

        for x in 0..size {
            for y in 0..size {
                let n = biome.get(x as i32, y as i32);
                assert!(n >= 0.0);
                assert!(n <= 1.0);
                let c = (n * 255.0) as u8;
                image.put_pixel(x, y, Rgb([c, c, c]));
            }
        }

        image.save("biome_100.png").unwrap();
    }
}
