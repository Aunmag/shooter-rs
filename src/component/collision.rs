use crate::util::ext::Vec2Ext;
use bevy::ecs::component::Component;
use bevy::math::Vec2;

const EXTRA_RESOLVE_DISTANCE: f32 = 0.0001;

#[derive(Component)]
pub struct Collision {
    pub radius: f32,
}

impl Collision {
    pub fn resolve(c1: &Self, c2: &Self, p1: Vec2, p2: Vec2) -> Option<Vec2> {
        let distance_squared = Vec2::distance_squared(p1, p2);
        let distance_min = c1.radius + c2.radius;

        if distance_squared < distance_min * distance_min {
            let distance = distance_squared.sqrt();
            let distance_to_push = (distance_min - distance) / 2.0 + EXTRA_RESOLVE_DISTANCE;
            let angle = p2.angle_to(p1);
            return Some(Vec2::from_length(distance_to_push, angle));
        } else {
            return None;
        }
    }
}

pub struct CollisionSolution {
    pub entity_index: u32,
    pub shift: Vec2,
    pub push: Vec2,
}
