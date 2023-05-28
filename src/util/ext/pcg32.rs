use rand::Rng;
use rand_distr::StandardNormal;
use rand_pcg::Pcg32;

pub trait Pcg32Ext {
    fn gen_range_safely(&mut self, min: f32, max: f32) -> f32;

    fn gen_normal(&mut self, deviation: f32) -> f32;
}

impl Pcg32Ext for Pcg32 {
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
