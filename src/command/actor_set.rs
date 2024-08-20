use crate::{
    component::{Actor, ActorConfig, ActorKind},
    data::{LAYER_ACTOR, TRANSFORM_SCALE},
    plugin::{collision::Collision, kinetics::Kinetics, Breath, Footsteps, Health},
    resource::Settings,
};
use bevy::{
    ecs::world::Command,
    math::{Quat, Vec2},
    prelude::{AssetServer, Entity, SpriteBundle, Transform, World},
};

pub struct ActorSet {
    pub entity: Entity,
    pub config: &'static ActorConfig,
    pub position: Vec2,
    pub rotation: f32,
}

impl Command for ActorSet {
    fn apply(self, world: &mut World) {
        let difficulty = world.resource::<Settings>().game.difficulty;
        let texture_path = self.config.get_image_path(0);
        let texture = world
            .resource::<AssetServer>()
            .get_handle(texture_path)
            .unwrap_or_default();

        let mut entity_mut = world.entity_mut(self.entity);

        entity_mut
            .insert(SpriteBundle {
                transform: Transform {
                    translation: self.position.extend(LAYER_ACTOR),
                    rotation: Quat::from_rotation_z(self.rotation),
                    scale: TRANSFORM_SCALE,
                },
                texture,
                ..Default::default()
            })
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
