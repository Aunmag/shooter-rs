use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;

pub struct Actor;

impl Actor {
    pub const MOVEMENT_VELOCITY: f32 = 2.0;
}

impl Component for Actor {
    type Storage = DenseVecStorage<Self>;
}
