use crate::{
    data::{LAYER_ACTOR, TRANSFORM_SCALE},
    plugin::{
        actor::{action::ActorActions, config::ActorKind, Actor, ActorConfig},
        bot::Bot,
        camera_target::CameraTarget,
        collision::Collision,
        kinetics::Kinetics,
        player::Player,
        Breath, Footsteps, Health, StatusBar,
    },
    resource::Settings,
};
use bevy::{
    ecs::{hierarchy::Children, system::Command},
    math::{Quat, Vec2},
    prelude::{AssetServer, Entity, Transform, World},
    sprite::Sprite,
    sprite_render::MeshMaterial2d,
};

pub struct ActorSet {
    pub entity: Entity,
    pub config: &'static ActorConfig,
    pub position: Vec2,
    pub rotation: f32,
}

impl Command for ActorSet {
    type Out = ();

    fn apply(self, world: &mut World) {
        let difficulty = world.resource::<Settings>().game.difficulty;
        let image_path = self.config.get_image_path(0);
        let image = world
            .resource::<AssetServer>()
            .get_handle(image_path)
            .unwrap_or_default();

        let mut entity_mut = world.entity_mut(self.entity);

        entity_mut
            .insert((
                Transform {
                    translation: self.position.extend(LAYER_ACTOR),
                    rotation: Quat::from_rotation_z(self.rotation),
                    scale: TRANSFORM_SCALE,
                },
                Sprite {
                    image,
                    ..Default::default()
                },
            ))
            .insert(Collision {
                radius: self.config.radius,
            })
            .insert(Kinetics::new(self.config.mass))
            .insert(Actor::new(self.config, difficulty))
            .insert(Health::new(self.config.health))
            .insert(Footsteps::default());

        if let ActorKind::Human = self.config.kind {
            entity_mut.insert(Breath::default());
        }
    }
}

// TODO: reset health multiplier
pub struct ActorRelease(pub Entity);

impl Command for ActorRelease {
    type Out = ();

    fn apply(self, world: &mut World) {
        let difficulty = world.resource::<Settings>().game.difficulty;

        // TODO: find a way to stop all sounds
        if let Some(actor) = world.get_mut::<Actor>(self.0).as_mut() {
            actor.movement = Vec2::ZERO;
            actor.actions = ActorActions::empty();
            actor.look_at = None;
            actor.skill = difficulty;
        }

        if let Some(crosshair) = world
            .get::<Player>(self.0)
            .and_then(|p| p.crosshair.as_ref())
        {
            world.entity_mut(crosshair.entity).despawn();
        }

        if let Some(kinetics) = world.get_mut::<Kinetics>(self.0).as_mut() {
            kinetics.drag = Kinetics::DRAG_DEFAULT;
        }

        let mut entity_mut = world.entity_mut(self.0);
        entity_mut.remove::<Bot>();
        entity_mut.remove::<Player>();
        entity_mut.remove::<Breath>();
        entity_mut.remove::<CameraTarget>();

        let mut to_remove = Vec::new();

        if let Some(children) = world.get::<Children>(self.0) {
            for &child in children {
                if world.get::<MeshMaterial2d<StatusBar>>(child).is_some() {
                    to_remove.push(child);
                }
            }
        }

        for entity in &to_remove {
            world.entity_mut(*entity).despawn();
        }
    }
}
