use rand::Rng;
use rand_distr::StandardNormal;
use rand_pcg::Pcg32;

pub trait Pcg32Ext {
    fn gen_normal(&mut self, deviation: f32) -> f32;
}

impl Pcg32Ext for Pcg32 {
    fn gen_normal(&mut self, deviation: f32) -> f32 {
        return (self.sample::<f32, _>(StandardNormal) - 0.5) * deviation;
    }
}
