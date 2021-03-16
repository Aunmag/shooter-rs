use crate::components::Actor;
use crate::components::Ai;
use crate::components::Collision;
use crate::components::Interpolation;
use crate::components::Player;
use crate::components::Projectile;
use crate::components::ProjectileConfig;
use crate::components::Terrain;
use crate::components::Weapon;
use crate::components::WeaponConfig;
use crate::data::LAYER_ACTOR;
use crate::data::LAYER_ACTOR_PLAYER;
use crate::data::LAYER_CAMERA;
use crate::data::LAYER_TERRAIN;
use crate::resources::EntityMap;
use crate::resources::Sprite;
use crate::resources::SpriteResource;
use crate::states::GameType;
use crate::utils::Position;
use amethyst::core::math::Vector2;
use amethyst::core::math::Vector3;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::core::Parent;
use amethyst::ecs::prelude::World;
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::Camera;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::Transparent;
use amethyst::tiles::MortonEncoder;
use amethyst::tiles::TileMap;

// TODO: Maybe name as `new_*` instead of `create_*`
// TODO: Maybe don't use `EntityMap`

pub fn get_entity(world: &World, external_id: u16) -> Option<Entity> {
    return world
        .read_resource::<EntityMap>()
        .get_entity(external_id)
        .filter(|e| world.is_alive(*e));
}

pub fn create_simple_sprite(
    world: &mut World,
    root: Entity,
    x: f32,
    y: f32,
    z: f32,
    direction: f32,
    sprite: SpriteRender,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, z);
    transform.set_rotation_2d(direction);

    return world
        .create_entity()
        .with(Parent { entity: root })
        .with(transform)
        .with(sprite)
        .build();
}

pub fn create_actor(
    world: &mut World,
    root: Entity,
    external_id: Option<u16>,
    position: Position,
    is_ghost: bool,
    game_type: &GameType,
) -> Entity {
    let now = world.read_resource::<Time>().absolute_time();

    let mut transform = Transform::default();
    transform.set_translation_xyz(position.x, position.y, LAYER_ACTOR);
    transform.set_rotation_2d(position.direction);

    let mut renderer = world
        .read_resource::<SpriteResource>()
        .get(Sprite::Actor)
        .map(|s| SpriteRender::new(s, 0));

    let mut builder = world
        .create_entity()
        .with(Parent { entity: root })
        .with(Actor::new())
        .with(transform)
        .with(Weapon::new(WeaponConfig {
            muzzle_velocity: 320.0,
            fire_rate: 650.0,
            projectile: ProjectileConfig {
                acceleration_factor: -7.0,
            },
        }));

    if let GameType::Join(..) = *game_type {
        builder = builder.with(Interpolation::new(position, now));
    }

    if let Some(renderer) = renderer.take() {
        builder = builder.with(renderer);
    }

    if is_ghost {
        builder = builder.with(Tint(Srgba::new(0.6, 0.6, 0.6, 0.4)));
        builder = builder.with(Transparent);
    } else {
        builder = builder.with(Collision { radius: 0.25 });
    }

    let actor = builder.build();

    if let Some(external_id) = external_id {
        world.fetch_mut::<EntityMap>().store(actor, external_id);
    }

    return actor;
}

pub fn grant_played_actor(world: &mut World, root: Entity, actor: Entity, game_type: &GameType) {
    // TODO: Remove old player entity
    // TODO: Reset layer for old transform
    // TODO: Remove old ghost
    // TODO: Remove old camera
    // TODO: Maybe make ghost as player's child

    let ghost;

    if let GameType::Join(..) = *game_type {
        ghost = Some(create_actor(
            world,
            root,
            None,
            Position::default(),
            true,
            game_type,
        ));
    } else {
        ghost = None;
    }

    if let Err(error) = world.write_storage::<Player>().insert(actor, Player::new(ghost)) {
        log::error!("Failed to insert Player component. Details: {}", error);
        // TODO: Remove the ghost then
    }

    if let Some(transform) = world.write_storage::<Transform>().get_mut(actor) {
        transform.set_translation_z(LAYER_ACTOR_PLAYER);
    }

    create_camera(world, actor);
}

pub fn set_actor_ai(world: &World, actor: Entity) {
    if let Err(error) = world.write_storage::<Ai>().insert(actor, Ai) {
        log::error!(
            "Failed to set AI for an actor ({}). Details: {}",
            actor.id(),
            error,
        );
    }
}

pub fn create_camera(world: &mut World, target: Entity) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, LAYER_CAMERA);

    return world
        .create_entity()
        .with(Camera::standard_2d(1.0, 1.0))
        .with(transform)
        .with(Parent { entity: target })
        .build();
}

pub fn create_terrain(world: &mut World, root: Entity) -> Entity {
    let quantity;

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    {
        quantity = Terrain::QUANTITY.abs().ceil() as u32;
    }

    let tile_map = TileMap::<Terrain, MortonEncoder>::new(
        Vector3::new(quantity, quantity, 1),
        Vector3::new(Terrain::SIZE, Terrain::SIZE, 1),
        world.read_resource::<SpriteResource>().get(Sprite::Grass),
    );

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, LAYER_TERRAIN);

    return world
        .create_entity()
        .with(Parent { entity: root })
        .with(tile_map)
        .with(transform)
        .build();
}

pub fn create_projectile(
    world: &mut World,
    root: Entity,
    position: Position,
    velocity: f32,
    acceleration_factor: f32,
    shooter_id: Option<u16>,
) -> Entity {
    let shooter = shooter_id
        .and_then(|id| world.read_resource::<EntityMap>().get_entity(id))
        .filter(|e| world.is_alive(*e));

    let (sin, cos) = (-position.direction).sin_cos();
    let projectile = Projectile::new(
        ProjectileConfig {
            acceleration_factor,
        },
        world.read_resource::<Time>().absolute_time(),
        Vector2::new(position.x, position.y),
        Vector2::new(velocity * sin, velocity * cos),
        shooter,
    );

    return world
        .create_entity()
        .with(Parent { entity: root })
        .with(projectile)
        .build();
}
