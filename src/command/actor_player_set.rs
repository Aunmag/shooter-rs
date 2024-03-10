use crate::{
    component::{Actor, Health, Inertia, Player},
    data::LAYER_ACTOR_PLAYER,
    plugin::{CameraTarget, Crosshair, LaserSight, StatusBar},
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
        let crosshair = Crosshair::spawn(world);
        let mut entity = world.entity_mut(self.entity);

        if let Some(mut actor) = entity.get_mut::<Actor>() {
            actor.skill = 1.0; // to keep game balance well, player skill must always be 1.0
        }

        if let Some(mut health) = entity.get_mut::<Health>() {
            health.multiply(health_multiplier);
        }

        if let Some(mut transform) = entity.get_mut::<Transform>() {
            transform.translation.z = LAYER_ACTOR_PLAYER;
        }

        if let Some(mut inertia) = entity.get_mut::<Inertia>() {
            inertia.drag = Inertia::DRAG_PLAYER;
        }

        // TODO: don't insert player if it isn't controllable
        entity.insert(Player::new(self.is_controllable, crosshair));
        entity.insert(CameraTarget::default());

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
