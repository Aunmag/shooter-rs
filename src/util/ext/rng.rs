use rand::Rng;
use rand_distr::StandardNormal;

pub trait RngExt {
    fn gen_range_safely(&mut self, min: f32, max: f32) -> f32;
    fn gen_normal(&mut self, deviation: f32) -> f32;
}

impl<R: Rng> RngExt for R {
    fn gen_range_safely(&mut self, mut min: f32, mut max: f32) -> f32 {
        if min > max {
            (min, max) = (max, min);
        }

        if min == max {
            return min;
        } else {
            return self.gen_range(min..max);
        }
    }

    fn gen_normal(&mut self, deviation: f32) -> f32 {
        return (self.sample::<f32, _>(StandardNormal) - 0.5) * deviation;
    }
}
