use bevy::ecs::entity::Entities;
use bevy::prelude::Entity;

#[derive(Default)]
pub struct EntityConverter {
    data: Vec<Record>,
}

struct Record {
    entity: Entity,
    external_id: u32,
}

impl EntityConverter {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_internal(&mut self, entities: &Entities, external_id: u32) -> Entity {
        for record in &self.data {
            if record.external_id == external_id {
                return record.entity;
            }
        }

        let entity = entities.reserve_entity();

        self.data.push(Record {
            entity,
            external_id,
        });

        return entity;
    }

    pub fn remove(&mut self, entity: Entity) {
        // TODO: to util
        for (i, record) in self.data.iter().enumerate() {
            if record.entity == entity {
                self.data.swap_remove(i);
                break;
            }
        }
    }
}
