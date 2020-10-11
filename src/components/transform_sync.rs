use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;

pub struct TransformSync {
    pub target_x: f32,
    pub target_y: f32,
    pub target_angle: f32,
}

impl TransformSync {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        return Self {
            target_x: x,
            target_y: y,
            target_angle: angle,
        };
    }
}

impl Component for TransformSync {
    type Storage = DenseVecStorage<Self>;
}
