use bevy::math::Vec2;

pub trait WithVelocity {
    fn velocity(&self) -> Vec2;

    /// Override if want to optimize
    fn velocity_linear(&self) -> f32 {
        return self.velocity().length();
    }
}
