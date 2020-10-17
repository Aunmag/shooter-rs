use amethyst::ecs::prelude::World;
use amethyst::ecs::Entity;
use amethyst::prelude::WorldExt;
use bimap::BiMap;

pub struct EntityIndexMap {
    map: BiMap<u32, u16>,
    last_generated_public_id: u16,
}

impl EntityIndexMap {
    pub fn new() -> Self {
        return Self {
            map: BiMap::new(),
            last_generated_public_id: 0,
        };
    }

    pub fn fetch_entity_by_public_id(world: &World, id: u16) -> Option<Entity> {
        return world
            .fetch::<Self>()
            .to_entity_id(id)
            .map(|id| world.entities().entity(id))
            .filter(|entity| world.is_alive(*entity));
    }

    pub fn store(&mut self, entity_id: u32, public_id: u16) {
        if public_id != 0 {
            self.map.insert(entity_id, public_id);
        }
    }

    pub fn generate_public_id(&mut self) -> u16 {
        self.last_generated_public_id = self.last_generated_public_id.wrapping_add(1);
        return self.last_generated_public_id;
    }

    pub fn to_entity_id(&self, id: u16) -> Option<u32> {
        return self.map.get_by_right(&id).copied();
    }

    pub fn to_public_id(&self, id: u32) -> Option<u16> {
        return self.map.get_by_left(&id).copied();
    }
}
