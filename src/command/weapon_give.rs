use crate::{
    component::{Actor, ActorWeaponSprite, Weapon, WeaponConfig},
    data::PIXELS_PER_METER,
    model::AudioPlay,
    resource::AudioTracker,
};
use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::system::Command,
    math::Vec3Swizzles,
    prelude::{BuildWorldChildren, Children, DespawnRecursiveExt, Entity, Transform, Vec2, World},
    render::texture::Image,
    sprite::{Anchor, Sprite, SpriteBundle},
};
use derive_more::Constructor;

#[derive(Constructor)]
pub struct WeaponGive {
    entity: Entity,
    weapon: &'static WeaponConfig,
}

impl WeaponGive {
    fn remove_old_weapon_sprite(&self, world: &mut World) {
        let mut to_remove = Vec::new();

        if let Some(children) = world.get::<Children>(self.entity) {
            for &child in children {
                if world.get::<ActorWeaponSprite>(child).is_some() {
                    to_remove.push(child);
                }
            }
        }

        world.entity_mut(self.entity).remove_children(&to_remove);

        for entity in &to_remove {
            world.entity_mut(*entity).despawn_recursive();
        }
    }

    fn spawn_weapon_sprite(&self, world: &mut World) {
        let image = world
            .resource::<AssetServer>()
            .get_handle(self.weapon.get_image_path());

        let anchor = self.find_image_anchor(world, &image);

        world
            .spawn(SpriteBundle {
                sprite: Sprite {
                    anchor,
                    ..Default::default()
                },
                texture: image,
                transform: Transform::from_xyz(0.0, 0.0, -0.1),
                ..Default::default()
            })
            .insert(ActorWeaponSprite)
            .set_parent(self.entity);
    }

    fn find_image_anchor(&self, world: &World, image: &Handle<Image>) -> Anchor {
        let arms_length = if self.weapon.actor_image_suffix == 1 {
            Actor::ARMS_LENGTH_1
        } else {
            Actor::ARMS_LENGTH_2
        };

        if let Some(image) = world.resource::<Assets<Image>>().get(image) {
            let offset = self.weapon.image_offset - arms_length * PIXELS_PER_METER;
            return Anchor::Custom(Vec2::new(offset / image.size().x - 0.5, 0.0));
        } else {
            log::warn!(
                "Unable to set anchor for image {} since it hasn't loaded yet",
                self.weapon.get_image_path(),
            );

            return Anchor::default();
        }
    }

    fn update_actor_weapon(&self, world: &mut World) {
        world
            .entity_mut(self.entity)
            .insert(Weapon::new(self.weapon));
    }

    fn update_actor_image(&self, world: &mut World) {
        if let Some(actor) = world.get::<Actor>(self.entity) {
            let image_suffix = self.weapon.actor_image_suffix;
            let image_path = actor.config.get_image_path(image_suffix);
            let image = world
                .resource::<AssetServer>()
                .get_handle::<Image, _>(image_path);
            world.entity_mut(self.entity).insert(image);
        }
    }

    fn play_pickup_sound(&self, world: &mut World) {
        if let Some(source) = world
            .get::<Transform>(self.entity)
            .map(|t| t.translation.xy())
        {
            world.resource_mut::<AudioTracker>().queue(AudioPlay {
                path: "sounds/pickup_weapon.ogg",
                volume: 0.9,
                source: Some(source),
                priority: AudioPlay::PRIORITY_HIGHER,
                ..AudioPlay::DEFAULT
            });
        }
    }
}

impl Command for WeaponGive {
    fn write(self, world: &mut World) {
        self.remove_old_weapon_sprite(world);
        self.spawn_weapon_sprite(world);
        self.update_actor_weapon(world);
        self.update_actor_image(world);
        self.play_pickup_sound(world);
    }
}
