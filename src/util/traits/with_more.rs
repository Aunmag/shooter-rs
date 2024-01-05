use crate::util::{
    math::find_meet_point,
    traits::{WithPosition, WithVelocity},
};
use bevy::math::Vec2;

pub trait WithPositionAndVelocity: WithPosition + WithVelocity {
    fn find_meet<T: WithPositionAndVelocity>(&self, target: &T) -> Vec2 {
        return find_meet_point(
            self.position(),
            self.velocity_linear(),
            target.position(),
            target.velocity(),
        );
    }
}

impl<T> WithPositionAndVelocity for T where T: WithPosition + WithVelocity {}
