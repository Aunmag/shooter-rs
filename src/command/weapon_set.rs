use crate::{
    component::{
        Actor, ActorWeaponSprite, Inertia, Voice, VoiceSound, Weapon, WeaponConfig, WeaponGrip,
    },
    data::PIXELS_PER_METER,
    model::AudioPlay,
    resource::AudioTracker,
};
use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::system::Command,
    math::Vec3Swizzles,
    prelude::{
        AudioBundle, BuildWorldChildren, Children, DespawnRecursiveExt, Entity, Transform, Vec2,
        World,
    },
    render::texture::Image,
    sprite::{Anchor, Sprite, SpriteBundle},
    time::Time,
};
use derive_more::Constructor;
use std::cmp::Ordering;

const WEAPON_MASS_MULTIPLAYER: f32 = 5.0;

#[derive(Constructor)]
pub struct WeaponSet {
    actor: Entity,
    weapon: Option<&'static WeaponConfig>,
    play_sound: bool,
}

impl WeaponSet {
    fn remove_old_weapon(&self, world: &mut World) -> Option<&'static WeaponConfig> {
        for children in world.get::<Children>(self.actor).iter() {
            for &child in children.iter() {
                if let Some(weapon) = world.get::<Weapon>(child).map(|w| w.config) {
                    world.entity_mut(child).despawn_recursive();
                    return Some(weapon);
                }
            }
        }

        return None;
    }

    fn spawn_weapon(&self, world: &mut World, weapon: &WeaponConfig) {
        let image = world
            .resource::<AssetServer>()
            .get_handle(weapon.get_image_path());

        let anchor = self.find_image_anchor(world, weapon, &image);

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
            .set_parent(self.actor);
    }

    fn find_image_anchor(
        &self,
        world: &World,
        weapon: &WeaponConfig,
        image: &Handle<Image>,
    ) -> Anchor {
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

    fn update_actor_mass(&self, world: &mut World, change: f32) {
        if let Some(inertia) = world.get_mut::<Inertia>(self.actor).as_mut() {
            inertia.mass += change * WEAPON_MASS_MULTIPLAYER;
        }
    }

    fn update_actor_image(&self, world: &mut World) {
        if let Some(actor) = world.get::<Actor>(self.actor) {
            let image_suffix = self
                .weapon
                .map(|w| w.grip.actor_image_suffix())
                .unwrap_or(0);

            let image_path = actor.config.get_image_path(image_suffix);

            let image = world
                .resource::<AssetServer>()
                .get_handle::<Image, _>(image_path);

            world.entity_mut(self.actor).insert(image);
        }
    }

    fn play_pickup_sound(&self, world: &mut World) {
        if let Some(source) = world
            .get::<Transform>(self.actor)
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

    fn play_actor_voice(&self, world: &mut World, weapon_previous: Option<&WeaponConfig>) {
        let is_shotgun = self.weapon.map_or(false, |w| w.partial_reloading);
        let level_previous = weapon_previous.map_or(0, |w| w.level);
        let level_current = self.weapon.map_or(0, |w| w.level);
        let is_same = false; // TODO: fix
        let is_worst = level_current == 1;
        let is_best = level_current == 6;

        let sound = match u8::cmp(&level_current, &level_previous) {
            Ordering::Less if is_worst => Some(VoiceSound::ArmWorse),
            Ordering::Less => Some(VoiceSound::ArmSimilar),
            Ordering::Equal if is_same => None,
            Ordering::Equal => Some(VoiceSound::ArmSimilar),
            Ordering::Greater if is_best => Some(VoiceSound::ArmBest),
            Ordering::Greater if is_shotgun => Some(VoiceSound::ArmShotgun),
            Ordering::Greater => Some(VoiceSound::ArmBetter),
        };

        if let Some(sound) = sound {
            let now = world.resource::<Time>().elapsed();

            if let Some(voice) = world.get_mut::<Voice>(self.actor).as_mut() {
                voice.queue(sound, now);
            }
        }
    }
}

impl Command for WeaponSet {
    fn apply(self, world: &mut World) {
        let weapon_previous = self.remove_old_weapon(world);

        if let Some(weapon) = weapon_previous {
            self.update_actor_mass(world, -weapon.get_mass_with_full_ammo());
        }

        if let Some(weapon) = self.weapon {
            self.spawn_weapon(world, weapon);
            self.update_actor_mass(world, weapon.get_mass_with_full_ammo());
        }

        self.update_actor_image(world);

        if self.play_sound {
            self.play_pickup_sound(world);
            self.play_actor_voice(world, weapon_previous);
        }
    }
}
