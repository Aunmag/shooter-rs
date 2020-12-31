use amethyst::ecs::Component;
use amethyst::ecs::NullStorage;

#[derive(Default)]
pub struct TransformSync;

impl Component for TransformSync {
    type Storage = NullStorage<Self>;
}
