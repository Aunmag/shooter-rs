use crate::{
    component::{Actor, Bot, Breath, Player},
    material::StatusBarMaterial,
    model::ActorActions,
};
use bevy::{
    asset::Handle,
    ecs::system::Command,
    hierarchy::{Children, DespawnRecursiveExt},
    math::Vec2,
    prelude::{Entity, World},
};

pub struct ActorRelease(pub Entity);

impl Command for ActorRelease {
    fn apply(self, world: &mut World) {
        // TODO: find a way to stop all sounds
        if let Some(actor) = world.get_mut::<Actor>(self.0).as_mut() {
            actor.movement = Vec2::ZERO;
            actor.actions = ActorActions::EMPTY;
            actor.look_at = None;
        }

        let mut entity_mut = world.entity_mut(self.0);
        entity_mut.remove::<Bot>();
        entity_mut.remove::<Player>();
        entity_mut.remove::<Breath>();

        let mut to_remove = Vec::new();

        if let Some(children) = world.get::<Children>(self.0) {
            for &child in children {
                if world.get::<Handle<StatusBarMaterial>>(child).is_some() {
                    to_remove.push(child);
                }
            }
        }

        for entity in &to_remove {
            world.entity_mut(*entity).despawn_recursive();
        }
    }
}
