use crate::{
    command::WeaponSet,
    component::{
        Actor, ActorConfig, ActorKind, Breath, Collision, Footsteps, Health, Inertia, Voice,
        WeaponConfig,
    },
    data::LAYER_ACTOR,
    model::TransformLite,
};
use bevy::{
    ecs::system::Command,
    prelude::{AssetServer, Entity, SpriteBundle, World},
    time::Time,
};

pub struct ActorSet {
    pub entity: Entity,
    pub config: &'static ActorConfig,
    pub skill: f32,
    pub transform: TransformLite,
}

impl Command for ActorSet {
    fn apply(self, world: &mut World) {
        let now = world.resource::<Time>().elapsed();
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
            .insert(Voice::new(now))
            .insert(Footsteps::default());

        if let ActorKind::Human = self.config.kind {
            entity_mut.insert(Breath::default());
            WeaponSet::new(self.entity, Some(&WeaponConfig::PM), false).apply(world);
        }
    }
}
