use crate::{
    component::{Actor, ActorWeaponSprite, Inertia, Weapon, WeaponConfig, WeaponGrip},
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

const WEAPON_MASS_MULTIPLAYER: f32 = 5.0;

#[derive(Constructor)]
pub struct WeaponSet {
    entity: Entity,
    weapon: Option<&'static WeaponConfig>,
}

impl WeaponSet {
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

    fn remove_old_weapon_component(&self, world: &mut World) {
        if let Some(weapon) = world.get::<Weapon>(self.entity) {
            let weapon_mass = weapon.config.get_mass_with_full_ammo();
            self.update_actor_mass(world, -weapon_mass);
            world.entity_mut(self.entity).remove::<Weapon>();
        }
    }

    fn spawn_weapon_sprite(&self, world: &mut World, weapon: &WeaponConfig) {
        let image = world
            .resource::<AssetServer>()
            .get_handle(weapon.get_image_path());

        let anchor = Self::find_image_anchor(world, weapon, &image);

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

    fn find_image_anchor(world: &World, weapon: &WeaponConfig, image: &Handle<Image>) -> Anchor {
        let arms_length = if let WeaponGrip::OneHand = weapon.grip {
            Actor::ARMS_LENGTH_1
        } else {
            Actor::ARMS_LENGTH_2
        };

        if let Some(image) = world.resource::<Assets<Image>>().get(image) {
            let offset = weapon.image_offset - arms_length * PIXELS_PER_METER;
            return Anchor::Custom(Vec2::new(offset / image.size().x - 0.5, 0.0));
        } else {
            log::warn!(
                "Unable to set anchor for image {} since it hasn't loaded yet",
                weapon.get_image_path(),
            );

            return Anchor::default();
        }
    }

    fn update_actor_weapon(&self, world: &mut World, weapon: &'static WeaponConfig) {
        world.entity_mut(self.entity).insert(Weapon::new(weapon));
    }

    fn update_actor_image(&self, world: &mut World, weapon: &WeaponConfig) {
        if let Some(actor) = world.get::<Actor>(self.entity) {
            let image_suffix = weapon.grip.actor_image_suffix();
            let image_path = actor.config.get_image_path(image_suffix);
            let image = world
                .resource::<AssetServer>()
                .get_handle::<Image, _>(image_path);
            world.entity_mut(self.entity).insert(image);
        }
    }

    fn update_actor_mass(&self, world: &mut World, change: f32) {
        if let Some(inertia) = world.get_mut::<Inertia>(self.entity).as_mut() {
            inertia.mass += change * WEAPON_MASS_MULTIPLAYER;
        }
    }

    fn play_pickup_sound(&self, world: &mut World) {
        if let Some(source) = world
            .get::<Transform>(self.entity)
            .map(|t| t.translation.xy())
        {
            world.resource_mut::<AudioTracker>().queue(AudioPlay {
                path: "sounds/pickup_weapon".into(),
                volume: 0.9,
                source: Some(source),
                ..AudioPlay::DEFAULT
            });
        }
    }
}

impl Command for WeaponSet {
    fn apply(self, world: &mut World) {
        self.remove_old_weapon_component(world);
        self.remove_old_weapon_sprite(world);

        if let Some(weapon) = self.weapon {
            self.spawn_weapon_sprite(world, weapon);
            self.update_actor_weapon(world, weapon);
            self.update_actor_image(world, weapon);
            self.update_actor_mass(world, weapon.get_mass_with_full_ammo());
            self.play_pickup_sound(world);
        }
    }
}
