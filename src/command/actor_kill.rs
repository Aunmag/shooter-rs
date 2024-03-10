use super::ActorRelease;
use crate::{
    component::{Actor, Collision, Health, Weapon},
    plugin::{AnimationConfig, BloodSpawn, Breath, Footsteps},
};
use bevy::{
    ecs::system::Command,
    prelude::{Entity, World},
    transform::components::Transform,
};

pub struct ActorKill(pub Entity);

impl Command for ActorKill {
    fn apply(self, world: &mut World) {
        ActorRelease(self.0).apply(world);

        let mut entity = world.entity_mut(self.0);
        entity.remove::<Collision>();
        entity.remove::<Actor>();
        entity.remove::<Health>();
        entity.remove::<Footsteps>();
        entity.remove::<Breath>();
        entity.remove::<Weapon>();
        entity.insert(AnimationConfig::ZOMBIE_DEATH.to_component()); // TODO: only for zombie?
                                                                     // TODO: remove all children?

        if let Some(transform) = entity.get::<Transform>() {
            if let Some(blood_spawn) = BloodSpawn::new(transform.translation.truncate(), 0.75) {
                blood_spawn.apply(world);
            }
        }
    }
}
