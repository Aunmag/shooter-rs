use crate::{
    component::{Actor, ActorConfig, ActorKind, Breath, Collision, Footsteps, Health, Inertia},
    data::LAYER_ACTOR,
    model::TransformLite,
    resource::Config,
};
use bevy::{
    ecs::system::Command,
    prelude::{AssetServer, Entity, SpriteBundle, World},
};

pub struct ActorSet {
    pub entity: Entity,
    pub config: &'static ActorConfig,
    pub transform: TransformLite,
}

impl Command for ActorSet {
    fn apply(self, world: &mut World) {
        let difficulty = world.resource::<Config>().game.difficulty;
        let texture_path = self.config.get_image_path(0);
        let texture = world
            .resource::<AssetServer>()
            .get_handle(texture_path)
            .unwrap_or_default();

        let mut entity_mut = world.entity_mut(self.entity);

        entity_mut
            .insert(SpriteBundle {
                transform: self.transform.as_transform(LAYER_ACTOR),
                texture,
                ..Default::default()
            })
            .insert(Collision {
                radius: self.config.radius,
            })
            .insert(Inertia::new(self.config.mass))
            .insert(Actor::new(self.config, difficulty))
            .insert(Health::new(self.config.health))
            .insert(Footsteps::default());

        if let ActorKind::Human = self.config.kind {
            entity_mut.insert(Breath::default());
        }
    }
}
