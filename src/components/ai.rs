use amethyst::ecs::Component;
use amethyst::ecs::NullStorage;

#[derive(Default)]
pub struct Ai;

impl Component for Ai {
    type Storage = NullStorage<Self>;
}
