use super::WeaponSet;
use crate::component::Bonus;
use bevy::{
    ecs::system::Command,
    prelude::{DespawnRecursiveExt, Entity, World},
};
use derive_more::Constructor;

#[derive(Constructor)]
pub struct BonusActivate {
    bonus: Entity,
    recipient: Entity,
}

impl Command for BonusActivate {
    fn apply(self, world: &mut World) {
        let Some(weapon) = world.get::<Bonus>(self.bonus).map(|b| b.weapon) else {
            return;
        };

        WeaponSet {
            entity: self.recipient,
            weapon: Some(weapon),
        }
        .apply(world);

        world.entity_mut(self.bonus).despawn_recursive();
    }
}
