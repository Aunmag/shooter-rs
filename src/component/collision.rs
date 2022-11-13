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
            let (sin, cos) = p1.atan2_to(p2).sin_cos();
            return Some(Vec2::new(distance_to_push * cos, distance_to_push * sin));
        } else {
            return None;
        }
    }
}

pub struct CollisionSolution {
    pub entity_id: u32,
    pub shift: Vec2,
    pub push: Vec2,
}
