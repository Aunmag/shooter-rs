use crate::{
    component::{Actor, Inertia, Player},
    data::LAYER_ACTOR_PLAYER,
    plugin::{CameraTarget, Health, LaserSight, StatusBar},
    resource::{GameMode, Settings},
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
        let health_multiplier = 1.0 / world.resource::<Settings>().game.difficulty;

        if let Some(mut actor) = world.get_mut::<Actor>(self.entity) {
            actor.skill = 1.0; // to keep game balance well, player skill must always be 1.0
        }

        if let Some(mut health) = world.get_mut::<Health>(self.entity) {
            health.multiply_resistance(health_multiplier);
        }

        if let Some(mut transform) = world.get_mut::<Transform>(self.entity) {
            transform.translation.z = LAYER_ACTOR_PLAYER;
        }

        if let Some(mut inertia) = world.get_mut::<Inertia>(self.entity) {
            inertia.drag = Inertia::DRAG_PLAYER;
        }

        // TODO: don't insert player if it isn't controllable
        world
            .entity_mut(self.entity)
            .insert(Player::new(self.is_controllable))
            .insert(CameraTarget::default());

        StatusBar::spawn(world, self.entity);

        if world
            .resource::<Settings>()
            .game
            .modes
            .contains(&GameMode::LaserSight)
        {
            LaserSight::spawn(world, self.entity);
        }
    }
}
