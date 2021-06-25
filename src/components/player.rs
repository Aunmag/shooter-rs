use amethyst::ecs::Component;
use amethyst::ecs::Entity;
use amethyst::ecs::VecStorage;

pub struct Player {
    pub ghost: Option<Entity>,
}

impl Player {
    pub const fn new(ghost: Option<Entity>) -> Self {
        return Self { ghost };
    }
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}
