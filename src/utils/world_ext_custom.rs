use amethyst::ecs::Component;
use amethyst::ecs::Entity;
use amethyst::ecs::World;
use amethyst::ecs::WorldExt;

pub trait WorldExtCustom {
    fn add<T: Component>(&self, entity: Entity, component: T);
}

impl WorldExtCustom for World {
    fn add<T: Component>(&self, entity: Entity, component: T) {
        if let Err(error) = self.write_storage::<T>().insert(entity, component) {
            log::error!(
                "Failed to insert a component for Entity({}): {}",
                entity.id(),
                error,
            );
        }
    }
}
