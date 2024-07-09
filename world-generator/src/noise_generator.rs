use std::f32::consts::{PI, TAU};

use rand::SeedableRng;
use rand_pcg::Pcg32;
use rand::Rng;

use crate::util::interpolate;

const OCTAVES: u32 = 4;
const OCTAVES_M: f32 = 2_i32.pow(OCTAVES) as f32;

pub struct NoiseGenerator {
    pub seed: u64,
    pub size: i32,
}

impl NoiseGenerator {
    pub fn gen_smooth_with_octaves(&self, x: f32, y: f32) -> f32 {
        let mut sum = 0.0;

        for i in 0..OCTAVES {
            let frequency = 2_i32.pow(i) as f32 / OCTAVES_M;
            sum += self.gen_smooth_f32(x * frequency, y * frequency) / OCTAVES as f32;
        }

        return sum;
    }

    pub fn gen_smooth_i32(&self, x: i32, y: i32) -> f32 {
        let s = self.size;
        let mut n = 0.0;
        // TODO: wrap add

        // center
        n += self.gen(x, y) / 4.0;

        // sides
        n += self.gen(x - s, y) / 8.0;
        n += self.gen(x + s, y) / 8.0;
        n += self.gen(x, y - s) / 8.0;
        n += self.gen(x, y + s) / 8.0;

        // corners
        n += self.gen(x - s, y - s) / 16.0;
        n += self.gen(x + s, y - s) / 16.0;
        n += self.gen(x - s, y + s) / 16.0;
        n += self.gen(x + s, y + s) / 16.0;

        return n;
    }

    pub fn gen_smooth_f32(&self, x: f32, y: f32) -> f32 {
        let x0 = x as i32;
        let x1 = x0 + 1;
        let y0 = y as i32;
        let y1 = y0 + 1;

        let xf = x.fract();
        let yf = y.fract();

        let n0 = self.gen_smooth_i32(x0, y0);
        let n1 = self.gen_smooth_i32(x1, y0);
        let i1 = interpolate(n0, n1, xf);

        let n0 = self.gen_smooth_i32(x0, y1);
        let n1 = self.gen_smooth_i32(x1, y1);
        let i2 = interpolate(n0, n1, xf);

        return interpolate(i1, i2, yf);
    }

    pub fn gen(&self, x: i32, y: i32) -> f32 {
        let seed = self.seed
            .wrapping_add_signed(i64::from(x / self.size * self.size).wrapping_mul(12345))
            .wrapping_add_signed(i64::from(y / self.size * self.size).wrapping_mul(98765))
            ;

        return Pcg32::seed_from_u64(seed).gen();
    }
}
