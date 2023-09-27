use bevy::{
    ecs::system::Resource,
    prelude::{Entity, Vec2},
};
use derive_more::Constructor;

#[derive(Default, Resource)]
pub struct HitResource {
    pub hits: Vec<HitTarget>,
}

impl HitResource {
    pub fn add(&mut self, attacker: Option<Entity>, entity: Entity, momentum: Vec2, angle: f32) {
        self.hits
            .push(HitTarget::new(attacker, entity, momentum, angle));
    }
}

#[derive(Constructor)]
pub struct HitTarget {
    pub attacker: Option<Entity>,
    pub entity: Entity,
    pub momentum: Vec2,
    pub angle: f32,
}
