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
    fn write(self, world: &mut World) {
        let weapon = if let Some(weapon) = world.get::<Bonus>(self.bonus).map(|b| b.weapon) {
            weapon
        } else {
            return;
        };

        WeaponSet::new(self.recipient, Some(weapon)).write(world);
        world.entity_mut(self.bonus).despawn_recursive();
    }
}
