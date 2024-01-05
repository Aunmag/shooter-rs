use crate::util::traits::{WithPosition, WithVelocity};
use bevy::math::Vec2;
use derive_more::Constructor;

#[derive(Constructor)]
pub struct Moving {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl WithPosition for Moving {
    fn position(&self) -> Vec2 {
        return self.position;
    }
}

impl WithVelocity for Moving {
    fn velocity(&self) -> Vec2 {
        return self.velocity;
    }
}

#[derive(Constructor)]
pub struct MovingSimple {
    pub position: Vec2,
    pub velocity: f32,
}

impl WithPosition for MovingSimple {
    fn position(&self) -> Vec2 {
        return self.position;
    }
}

impl WithVelocity for MovingSimple {
    fn velocity(&self) -> Vec2 {
        unreachable!();
    }

    fn velocity_linear(&self) -> f32 {
        return self.velocity;
    }
}
