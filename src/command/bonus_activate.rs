use super::AudioPlay;
use crate::component::{Bonus, Weapon};
use bevy::{
    ecs::system::Command,
    math::Vec3Swizzles,
    prelude::{DespawnRecursiveExt, Entity, Transform, World},
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

        if let Some(transform) = world.get::<Transform>(self.recipient) {
            AudioPlay {
                path: "sounds/pickup_weapon.ogg",
                volume: 0.9,
                source: Some(transform.translation.xy()),
                ..AudioPlay::DEFAULT
            }
            .write(world);
        }

        world.entity_mut(self.recipient).insert(Weapon::new(weapon));
        world.entity_mut(self.bonus).despawn_recursive();
    }
}
