use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;
use amethyst::ecs::Entity;

pub struct Player {
    pub ghost: Option<Entity>,
}

impl Player {
    pub fn new(ghost: Option<Entity>) -> Self {
        return Self { ghost };
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
