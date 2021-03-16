use crate::utils::math;
use amethyst::core::math::Vector2;
use amethyst::ecs::Component;
use amethyst::ecs::VecStorage;

const EXTRA_RESOLVE_DISTANCE: f32 = 0.0001;

pub struct Collision {
    pub radius: f32,
}

impl Collision {
    pub fn resolve(
        c1: &Self,
        c2: &Self,
        p1: Vector2<f32>,
        p2: Vector2<f32>,
    ) -> Option<Vector2<f32>> {
        let distance_squared = math::distance_squared(p1.x, p1.y, p2.x, p2.y);
        let distance_min = c1.radius + c2.radius;

        if distance_squared < distance_min * distance_min {
            let distance = distance_squared.sqrt();
            let distance_to_push = (distance_min - distance) / 2.0 + EXTRA_RESOLVE_DISTANCE;
            let (sin, cos) = math::angle(p1.x, p1.y, p2.x, p2.y).sin_cos();

            return Some(Vector2::new(distance_to_push * cos, distance_to_push * sin));
        } else {
            return None;
        }
    }
}

impl Component for Collision {
    type Storage = VecStorage<Self>;
}
