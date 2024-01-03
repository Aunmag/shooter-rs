use super::LaserSightSet;
use crate::{
    command::StatusBarSet,
    component::Player,
    data::LAYER_ACTOR_PLAYER,
    resource::{Config, GameMode},
};
use bevy::{
    ecs::system::Command,
    prelude::{Entity, Transform, World},
};

pub struct ActorPlayerSet {
    pub entity: Entity,
    pub is_controllable: bool,
}

impl Command for ActorPlayerSet {
    fn apply(self, world: &mut World) {
        if let Some(mut transform) = world.get_mut::<Transform>(self.entity) {
            transform.translation.z = LAYER_ACTOR_PLAYER;
        }

        world
            .entity_mut(self.entity)
            .insert(Player::new(self.is_controllable));

        StatusBarSet(self.entity).apply(world);

        if world
            .resource::<Config>()
            .game
            .modes
            .contains(&GameMode::LaserSight)
        {
            LaserSightSet(self.entity).apply(world);
        }
    }
}
