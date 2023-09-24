use crate::{
    component::{Bonus, BonusImage, BonusLabel, WeaponConfig},
    data::{FONT_PATH, LAYER_BONUS, PIXELS_PER_METER, TRANSFORM_SCALE},
};
use bevy::{
    ecs::system::Command,
    prelude::{
        AssetServer, BuildWorldChildren, Color, Entity, SpatialBundle, SpriteBundle, Transform,
        Vec2, Vec3, World,
    },
    text::{Text, Text2dBundle, TextAlignment, TextStyle},
    time::Time,
};
use derive_more::Constructor;
use rand::seq::SliceRandom;

#[derive(Constructor)]
pub struct BonusSpawn {
    position: Vec2,
    level: u8,
}

impl Command for BonusSpawn {
    fn apply(self, world: &mut World) {
        let weapon = if let Some(weapon) = generate_weapon(self.level) {
            weapon
        } else {
            return;
        };

        let bonus = spawn_bonus(world, self.position, weapon);
        spawn_image(world, bonus, weapon);
        spawn_label(world, bonus, weapon);
    }
}

fn spawn_bonus(world: &mut World, position: Vec2, weapon: &'static WeaponConfig) -> Entity {
    let time = world.resource::<Time>().elapsed();

    return world
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(position.x, position.y, LAYER_BONUS)
                .with_scale(TRANSFORM_SCALE),
            ..Default::default()
        })
        .insert(Bonus::new(weapon, time))
        .id();
}

fn spawn_image(world: &mut World, bonus: Entity, weapon: &'static WeaponConfig) {
    let texture = world
        .resource::<AssetServer>()
        .get_handle(weapon.get_image_path());

    world
        .spawn(SpriteBundle {
            texture,
            ..Default::default()
        })
        .insert(BonusImage)
        .set_parent(bonus);
}

fn spawn_label(world: &mut World, bonus: Entity, weapon: &'static WeaponConfig) {
    let font = world.resource::<AssetServer>().get_handle(FONT_PATH);
    let text = Text::from_section(
        weapon.name,
        TextStyle {
            font,
            font_size: PIXELS_PER_METER,
            color: Color::WHITE,
        },
    )
    .with_alignment(TextAlignment::Center);

    world
        .spawn(Text2dBundle {
            transform: Transform::from_scale(Vec3::ZERO),
            text,
            ..Default::default()
        })
        .insert(BonusLabel)
        .set_parent(bonus);
}

fn generate_weapon(level: u8) -> Option<&'static WeaponConfig> {
    return WeaponConfig::ALL
        .choose_weighted(&mut rand::thread_rng(), |weapon| {
            if weapon.level > level {
                return 0.0;
            } else {
                return 1.0;
            }
        })
        .ok();
}
