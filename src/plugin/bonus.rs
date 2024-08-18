use crate::{
    component::{Actor, ActorKind},
    data::{FONT_PATH, LAYER_BONUS, PIXELS_PER_METER, TRANSFORM_SCALE},
    model::AppState,
    plugin::{
        camera::MainCamera, collision::Collision, player::Player, Weapon, WeaponConfig, WeaponSet,
    },
    util::{
        ext::{AppExt, Vec2Ext},
        math::interpolate,
    },
};
use bevy::{
    app::{App, Plugin},
    color::palettes::css::WHITE,
    ecs::{component::Component, entity::Entity, system::Res, world::Command},
    math::Vec3Swizzles,
    prelude::{
        AssetServer, BuildWorldChildren, Commands, DespawnRecursiveExt, IntoSystemConfigs, Quat,
        Query, SpatialBundle, SpriteBundle, Vec2, Vec3, With, Without, World,
    },
    text::{JustifyText, Text, Text2dBundle, TextStyle},
    time::Time,
    transform::components::Transform,
};
use rand::seq::SliceRandom;
use std::{f32::consts::TAU, time::Duration};

const RADIUS: f32 = 0.2;
const PULSE: Duration = Duration::from_secs(2);
const TEXT_SCALE_MIN: f32 = 0.39;
const TEXT_SCALE_MAX: f32 = 0.41;
const LIFETIME: Duration = Duration::from_secs(30);

pub struct BonusPlugin;

impl Plugin for BonusPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(
            AppState::Game,
            update_pickup.after(crate::plugin::collision::on_update),
        );
        app.add_state_system(AppState::Game, update_image);
        app.add_state_system(AppState::Game, update_label);
    }
}

pub struct BonusSpawn {
    position: Vec2,
    level: u8,
}

impl BonusSpawn {
    pub fn new(position: Vec2, level: u8) -> Self {
        return Self { position, level };
    }
}

impl Command for BonusSpawn {
    fn apply(self, world: &mut World) {
        if let Some(weapon) = choose_weapon(world, self.level) {
            let bonus = spawn_bonus(world, self.position, weapon);
            spawn_image(world, bonus, weapon);
            spawn_label(world, bonus, weapon);
        }
    }
}

#[derive(Component)]
struct Bonus {
    weapon: &'static WeaponConfig,
    expiration: Duration,
}

#[derive(Component)]
struct BonusImage;

#[derive(Component)]
struct BonusLabel;

fn update_pickup(
    bonuses: Query<(Entity, &Bonus, &Transform)>,
    players: Query<(Entity, &Actor, &Transform, &Collision), With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let now = time.elapsed();

    for (bonus_entity, bonus, bonus_transform) in bonuses.iter() {
        if now > bonus.expiration {
            commands.entity(bonus_entity).despawn_recursive();
            continue;
        }

        let bonus_position = bonus_transform.translation.xy();

        for (player_entity, actor, player_transform, player_body) in players.iter() {
            if actor.config.kind != ActorKind::Human {
                continue;
            }

            let player_position = player_transform.translation.xy();

            if player_position.is_close(bonus_position, RADIUS + player_body.radius) {
                commands.entity(bonus_entity).despawn_recursive();
                commands.add(WeaponSet {
                    entity: player_entity,
                    weapon: Some(bonus.weapon),
                });

                break;
            }
        }
    }
}

fn update_image(mut query: Query<&mut Transform, With<BonusImage>>, time: Res<Time>) {
    let update_image = time.elapsed_seconds() % PULSE.as_secs_f32() * -TAU;
    for mut image in query.iter_mut() {
        image.rotation = Quat::from_rotation_z(update_image);
    }
}

fn update_label(
    mut query: Query<&mut Transform, With<BonusLabel>>,
    cameras: Query<&Transform, (With<MainCamera>, Without<BonusLabel>)>,
    time: Res<Time>,
) {
    let scale = interpolate(
        TEXT_SCALE_MIN,
        TEXT_SCALE_MAX,
        (time.elapsed_seconds() / PULSE.as_secs_f32() * TAU * 2.0).cos(),
    );

    let rotation = cameras
        .iter()
        .next()
        .map_or_else(Default::default, |c| c.rotation);

    for mut label in query.iter_mut() {
        label.translation = rotation * Vec3::new(0.0, -5.0, 1.5);
        label.rotation = rotation;
        label.scale = Vec3::splat(scale);
    }
}

fn choose_weapon(world: &mut World, level: u8) -> Option<&'static WeaponConfig> {
    let mut weapon_of_all_the_players = None;

    for weapon in world
        .query_filtered::<Option<&Weapon>, With<Player>>()
        .iter(world)
    {
        let weapon_name = weapon.map(|w| w.config.name).unwrap_or("");

        match weapon_of_all_the_players.map(|w| weapon_name == w) {
            None => {
                // remember the weapon of first encountered player
                weapon_of_all_the_players = Some(weapon_name);
            }
            Some(true) => {
                // one more player has same weapon
                continue;
            }
            Some(false) => {
                // one player has a different weapon
                weapon_of_all_the_players = None;
                break;
            }
        }
    }

    return WeaponConfig::ALL
        .choose_weighted(&mut rand::thread_rng(), |w| {
            if w.level > level || Some(w.name) == weapon_of_all_the_players {
                return 0.0;
            } else if w.level == 1 {
                return 0.1; // less pistols, they usually get in the way
            } else {
                return 1.0;
            }
        })
        .ok();
}

fn spawn_bonus(world: &mut World, position: Vec2, weapon: &'static WeaponConfig) -> Entity {
    let time = world.resource::<Time>().elapsed();

    return world
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(position.x, position.y, LAYER_BONUS)
                .with_scale(TRANSFORM_SCALE),
            ..Default::default()
        })
        .insert(Bonus {
            weapon,
            expiration: time + LIFETIME,
        })
        .id();
}

fn spawn_image(world: &mut World, bonus: Entity, weapon: &'static WeaponConfig) {
    let texture = world
        .resource::<AssetServer>()
        .get_handle(weapon.get_image_path())
        .unwrap_or_default();

    world
        .spawn(SpriteBundle {
            texture,
            ..Default::default()
        })
        .insert(BonusImage)
        .set_parent(bonus);
}

fn spawn_label(world: &mut World, bonus: Entity, weapon: &'static WeaponConfig) {
    let font = world
        .resource::<AssetServer>()
        .get_handle(FONT_PATH)
        .unwrap_or_default();

    let text = Text::from_section(
        weapon.name,
        TextStyle {
            font,
            font_size: PIXELS_PER_METER,
            color: WHITE.into(),
        },
    )
    .with_justify(JustifyText::Center);

    world
        .spawn(Text2dBundle {
            transform: Transform::from_scale(Vec3::ZERO),
            text,
            ..Default::default()
        })
        .insert(BonusLabel)
        .set_parent(bonus);
}
