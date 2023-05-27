use crate::{
    component::{
        Actor, ActorConfig, ActorType, Breath, Collision, Footsteps, Health, Inertia, Weapon,
        WeaponConfig,
    },
    data::LAYER_ACTOR,
    model::TransformLite,
};
use bevy::{
    asset::Assets,
    ecs::system::Command,
    prelude::{AssetServer, Entity, Image, Sprite, SpriteBundle, World},
    sprite::Anchor,
};

pub struct ActorSet {
    pub entity: Entity,
    pub config: &'static ActorConfig,
    pub skill: f32,
    pub transform: TransformLite,
}

impl Command for ActorSet {
    fn write(self, world: &mut World) {
        let texture = world
            .resource::<AssetServer>()
            .get_handle(self.config.sprite);

        let anchor = if let Some(image) = world.resource::<Assets<Image>>().get(&texture) {
            self.config.sprite_offset.to_anchor(image)
        } else {
            log::warn!(
                "Unable to set anchor for sprite {} since it hasn't loaded yet",
                self.config.sprite,
            );

            Anchor::default()
        };

        let mut entity_mut = world.entity_mut(self.entity);

        entity_mut
            .insert(SpriteBundle {
                sprite: Sprite {
                    anchor,
                    ..Default::default()
                },
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
            entity_mut.insert(Weapon::new(&WeaponConfig::PM));
            entity_mut.insert(Breath::default());
        }
    }
}
