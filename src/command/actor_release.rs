use crate::{
    component::{Actor, Bot, Inertia, Player},
    model::ActorActions,
    plugin::{Breath, CameraTarget, StatusBar},
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

        // TODO: find a way to stop all sounds
        if let Some(actor) = world.get_mut::<Actor>(self.0).as_mut() {
            actor.movement = Vec2::ZERO;
            actor.actions = ActorActions::EMPTY;
            actor.look_at = None;
            actor.skill = difficulty;
        }

        if let Some(player) = world.get::<Player>(self.0) {
            world
                .entity_mut(player.crosshair.entity)
                .despawn_recursive();
        }

        if let Some(inertia) = world.get_mut::<Inertia>(self.0).as_mut() {
            inertia.drag = Inertia::DRAG_DEFAULT;
        }

        let mut entity_mut = world.entity_mut(self.0);
        entity_mut.remove::<Bot>();
        entity_mut.remove::<Player>();
        entity_mut.remove::<Breath>();
        entity_mut.remove::<CameraTarget>();

        let mut to_remove = Vec::new();

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
