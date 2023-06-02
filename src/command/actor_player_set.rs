use crate::{
    command::StatusBarSet,
    component::{Actor, ActorConfig, Inertia, Player},
    data::LAYER_ACTOR_PLAYER,
};
use bevy::{
    ecs::system::Command,
    prelude::{Entity, Transform, World},
};

pub struct ActorPlayerSet(pub Entity);

impl Command for ActorPlayerSet {
    fn write(self, world: &mut World) {
        if let Some(mut transform) = world.get_mut::<Transform>(self.0) {
            transform.translation.z = LAYER_ACTOR_PLAYER;
        }

        let config = world
            .get::<Actor>(self.0)
            .map(|a| a.config)
            .unwrap_or_else(|| {
                log::warn!("Couldn't find actor component");
                return &ActorConfig::HUMAN;
            });

        let mut entity_mut = world.entity_mut(self.0);
        entity_mut.insert(Player::default());
        entity_mut.insert(Inertia::new(config.mass));

        StatusBarSet(self.0).write(world);
    }
}
