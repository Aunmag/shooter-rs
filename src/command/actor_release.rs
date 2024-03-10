use crate::{
    component::{Actor, Inertia, Player},
    model::ActorActions,
    plugin::{bot::Bot, Breath, CameraTarget, StatusBar},
    resource::Settings,
};
use bevy::{
    asset::Handle,
    ecs::system::Command,
    hierarchy::{Children, DespawnRecursiveExt},
    math::Vec2,
    prelude::{Entity, World},
};

// TODO: reset health multiplier
pub struct ActorRelease(pub Entity);

impl Command for ActorRelease {
    fn apply(self, world: &mut World) {
        let difficulty = world.resource::<Settings>().game.difficulty;

        // TODO: optimize?
        if let Some(player) = world.get::<Player>(self.0) {
            world
                .entity_mut(player.crosshair.entity)
                .despawn_recursive();
        }

        let mut entity = world.entity_mut(self.0);

        // TODO: find a way to stop all sounds
        if let Some(actor) = entity.get_mut::<Actor>().as_mut() {
            actor.movement = Vec2::ZERO;
            actor.actions = ActorActions::EMPTY;
            actor.look_at = None;
            actor.skill = difficulty;
        }

        if let Some(inertia) = entity.get_mut::<Inertia>().as_mut() {
            inertia.drag = Inertia::DRAG_DEFAULT;
        }

        entity.remove::<Bot>();
        entity.remove::<Player>();
        entity.remove::<Breath>();
        entity.remove::<CameraTarget>();

        let mut to_remove = Vec::new();

        // TODO: optimize?
        if let Some(children) = world.get::<Children>(self.0) {
            for &child in children {
                if world.get::<Handle<StatusBar>>(child).is_some() {
                    to_remove.push(child);
                }
            }
        }

        for entity in &to_remove {
            world.entity_mut(*entity).despawn_recursive();
        }
    }
}
