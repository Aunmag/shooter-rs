use crate::{
    command::WeaponGive,
    component::{
        Actor, ActorConfig, ActorType, Breath, Collision, Footsteps, Health, Inertia, WeaponConfig,
    },
    data::LAYER_ACTOR,
    model::TransformLite,
};
use bevy::{
    ecs::system::Command,
    prelude::{AssetServer, Entity, SpriteBundle, World},
};

pub struct ActorSet {
    pub entity: Entity,
    pub config: &'static ActorConfig,
    pub skill: f32,
    pub transform: TransformLite,
}

impl Command for ActorSet {
    fn write(self, world: &mut World) {
        let texture_path = self.config.get_image_path(0);
        let texture = world.resource::<AssetServer>().get_handle(texture_path);
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
            .insert(Actor::new(self.config, self.skill))
            .insert(Health::new(self.config.resistance * self.skill))
            .insert(Footsteps::default());

        if let ActorType::Human = self.config.actor_type {
            entity_mut.insert(Breath::default());
            WeaponGive::new(self.entity, &WeaponConfig::PM).write(world);
        }
    }
}
