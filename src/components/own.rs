use amethyst::ecs::Component;
use amethyst::ecs::NullStorage;

#[derive(Default)]
pub struct Own;

impl Component for Own {
    type Storage = NullStorage<Self>;
}
