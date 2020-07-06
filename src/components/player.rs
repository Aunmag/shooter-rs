use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;

pub struct Player;

impl Player {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
