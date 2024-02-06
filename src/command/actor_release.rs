use crate::{
    component::{Actor, Bot, Breath, Heartbeat, Player},
    material::StatusBarMaterial,
    model::ActorActions,
    plugin::CameraTarget,
    resource::Config,
};
use bevy::{
    asset::Handle,
    audio::AudioSink,
    ecs::{query::With, system::Command},
    hierarchy::{Children, DespawnRecursiveExt},
    math::Vec2,
    prelude::{AudioSinkPlayback, Entity, World},
};

// TODO: reset health multiplier
pub struct ActorRelease(pub Entity);

impl Command for ActorRelease {
    fn apply(self, world: &mut World) {
        let difficulty = world.resource::<Config>().game.difficulty;

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

        let mut entity_mut = world.entity_mut(self.0);
        entity_mut.remove::<Bot>();
        entity_mut.remove::<Player>();
        entity_mut.remove::<Breath>();
        entity_mut.remove::<CameraTarget>();

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

        for heartbeat in world
            .query_filtered::<&mut AudioSink, With<Heartbeat>>()
            .iter_mut(world)
        {
            heartbeat.pause();
        }
    }
}
