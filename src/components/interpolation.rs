use amethyst::ecs::Component;
use amethyst::ecs::VecStorage;

pub struct Interpolation {
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_angle: f32,
}

impl Interpolation {
    pub fn new() -> Self {
        return Self {
            offset_x: 0.0,
            offset_y: 0.0,
            offset_angle: 0.0,
        };
    }
}

impl Component for Interpolation {
    type Storage = VecStorage<Self>;
}
