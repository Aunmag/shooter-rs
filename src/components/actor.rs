use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;

pub struct Actor;

impl Actor {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Component for Actor {
    type Storage = DenseVecStorage<Self>;
}
