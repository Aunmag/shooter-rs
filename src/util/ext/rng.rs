use rand::{Rng, RngExt};
use rand_distr::StandardNormal;
use std::{cmp::Ordering, time::Duration};

pub trait RngExt2 {
    fn gen_range_safely(&mut self, min: f32, max: f32) -> f32;
    fn gen_normal(&mut self, deviation: f32) -> f32;
}

impl<R: Rng> RngExt2 for R {
    fn gen_range_safely(&mut self, min: f32, max: f32) -> f32 {
        match f32::partial_cmp(&min, &max) {
            Some(Ordering::Less) => {
                return self.random_range(min..max);
            }
            Some(Ordering::Greater) => {
                return self.random_range(max..min);
            }
            _ => {
                return min;
            }
        }
    }

    fn gen_normal(&mut self, deviation: f32) -> f32 {
        return (self.sample::<f32, _>(StandardNormal) - 0.5) * deviation;
    }
}

pub trait Fuzz {
    fn fuzz_with<R: Rng>(self, rng: &mut R, n: f32) -> Self;

    fn fuzz<R: Rng>(self, rng: &mut R) -> Self
    where
        Self: Sized,
    {
        return self.fuzz_with(rng, 0.4);
    }
}

impl Fuzz for f32 {
    fn fuzz_with<R: Rng>(self, rng: &mut R, n: f32) -> Self {
        return self * (1.0 + rng.random_range(-n..n));
    }
}

impl Fuzz for Duration {
    fn fuzz_with<R: Rng>(self, rng: &mut R, n: f32) -> Self {
        return self.mul_f32(1.0 + rng.random_range(-n..n));
    }
}
