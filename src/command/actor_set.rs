use crate::component::Actor;
use crate::component::ActorConfig;
use crate::component::Collision;
use crate::component::Health;
use crate::component::Inertia;
use crate::component::Interpolation;
use crate::component::ProjectileConfig;
use crate::component::Weapon;
use crate::component::WeaponConfig;
use crate::data::LAYER_ACTOR;
use crate::model::Position;
use crate::resource::Message;
use crate::resource::NetResource;
use crate::util::ext::WorldExt;
use bevy::asset::Assets;
use bevy::ecs::system::Command;
use bevy::prelude::AssetServer;
use bevy::prelude::Color;
use bevy::prelude::Entity;
use bevy::prelude::Image;
use bevy::prelude::Sprite;
use bevy::prelude::SpriteBundle;
use bevy::prelude::Time;
use bevy::prelude::World;
use bevy::sprite::Anchor;

pub struct ActorSet {
    pub entity: Entity,
    pub config: &'static ActorConfig,
    pub position: Position,
    pub is_ghost: bool,
}

impl Command for ActorSet {
    fn write(self, world: &mut World) {
        let is_server = world.is_server();

        if is_server {
            world
                .resource_mut::<NetResource>()
                .send_to_all(Message::ActorSpawn {
                    id: 0,
                    entity_index: self.entity.index(),
                    actor_type: self.config.actor_type,
                    position: self.position,
                });
        }

        let now = world.resource::<Time>().elapsed();
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

        let color = if self.is_ghost {
            Color::rgba(0.6, 0.6, 0.6, 0.4)
        } else {
            Color::default()
        };

        // TODO: reduce components on client

        entity_mut
            .insert(SpriteBundle {
                sprite: Sprite {
                    anchor,
                    color,
                    ..Default::default()
                },
                transform: self.position.as_transform(LAYER_ACTOR),
                texture,
                ..Default::default()
            })
            .insert(Actor::new(self.config))
            .insert(Weapon::new(WeaponConfig {
                muzzle_velocity: 320.0,
                fire_rate: 650.0,
                projectile: ProjectileConfig {
                    acceleration_factor: -7.0,
                },
            }));

        if is_server {
            entity_mut.insert(Health::new(self.config.resistance));
            entity_mut.insert(Inertia::new(self.config.mass));
        } else {
            entity_mut.insert(Interpolation::new(self.position, now));
        }

        if !self.is_ghost {
            entity_mut.insert(Collision {
                radius: self.config.radius,
            });
        }
    }
}
