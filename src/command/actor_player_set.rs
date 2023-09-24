use super::LaserSightSet;
use crate::{command::StatusBarSet, component::Player, data::LAYER_ACTOR_PLAYER, resource::Config};
use bevy::{
    ecs::system::Command,
    prelude::{Entity, Transform, World},
};

pub struct ActorPlayerSet(pub Entity);

impl Command for ActorPlayerSet {
    fn apply(self, world: &mut World) {
        if let Some(mut transform) = world.get_mut::<Transform>(self.0) {
            transform.translation.z = LAYER_ACTOR_PLAYER;
        }

        world.entity_mut(self.0).insert(Player::default());

        StatusBarSet(self.0).apply(world);

        if world.resource::<Config>().misc.laser_sight {
            LaserSightSet(self.0).apply(world);
        }
    }
}
