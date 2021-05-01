use crate::components::Actor;
use crate::components::Ai;
use crate::components::Collision;
use crate::components::Health;
use crate::components::Interpolation;
use crate::components::Own;
use crate::components::Player;
use crate::components::Projectile;
use crate::components::ProjectileConfig;
use crate::components::RigidBody;
use crate::components::Terrain;
use crate::components::Weapon;
use crate::components::WeaponConfig;
use crate::data::LAYER_ACTOR;
use crate::data::LAYER_ACTOR_PLAYER;
use crate::data::LAYER_CAMERA;
use crate::data::LAYER_TERRAIN;
use crate::models::GameType;
use crate::resources::Sprite;
use crate::resources::SpriteResource;
use crate::resources::State;
use crate::utils::Position;
use crate::utils::WorldExtCustom;
use amethyst::core::math::Vector2;
use amethyst::core::math::Vector3;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::core::Parent;
use amethyst::ecs::Entity;
use amethyst::ecs::World;
use amethyst::prelude::*;
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::Camera;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::Transparent;
use amethyst::tiles::MortonEncoder;
use amethyst::tiles::TileMap;

// TODO: Maybe name as `new_*` instead of `create_*`

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
    entity: Entity,
    position: Position,
    is_ghost: bool,
    game_type: &GameType,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(position.x, position.y, LAYER_ACTOR);
    transform.set_rotation_2d(position.direction);

    world.add(entity, transform);
    world.add(entity, Parent { entity: root });
    world.add(entity, Actor::new());
    world.add(
        entity,
        Weapon::new(WeaponConfig {
            muzzle_velocity: 320.0,
            fire_rate: 650.0,
            projectile: ProjectileConfig {
                acceleration_factor: -7.0,
            },
        }),
    );

    match *game_type {
        GameType::Server(..) => {
            world.add(entity, Own);
            world.add(entity, Health::new(Actor::RESISTANCE));
        }
        GameType::Client(..) => {
            let now = world.read_resource::<Time>().absolute_time();
            world.add(entity, Interpolation::new(position, now));
        }
    }

    if let Some(renderer) = world
        .read_resource::<SpriteResource>()
        .get(Sprite::Actor)
        .map(|s| SpriteRender::new(s, 0))
    {
        world.add(entity, renderer);
    }

    if is_ghost {
        world.add(entity, Tint(Srgba::new(0.6, 0.6, 0.6, 0.4)));
        world.add(entity, Transparent);
    } else {
        world.add(entity, Collision { radius: 0.25 });
        world.add(entity, RigidBody::new(80_000.0, 7.0, 8.0, 0.05));
    }

    return entity;
}

pub fn grant_played_actor(world: &mut World, root: Entity, actor: Entity, game_type: &GameType) {
    // TODO: Remove old player entity
    // TODO: Reset layer for old transform
    // TODO: Remove old ghost
    // TODO: Remove old camera
    // TODO: Remove old ownership
    // TODO: Maybe make ghost as player's child

    let ghost;

    match *game_type {
        GameType::Server(..) => {
            ghost = None;
        }
        GameType::Client(..) => {
            let entity = world.entities().create();

            ghost = Some(create_actor(
                world,
                root,
                entity,
                Position::default(),
                true,
                game_type,
            ));
        }
    }

    world.add(actor, Own);
    world.add(actor, Player::new(ghost));

    if let Some(transform) = world.write_storage::<Transform>().get_mut(actor) {
        transform.set_translation_z(LAYER_ACTOR_PLAYER);
    }

    create_camera(world, actor);
}

pub fn set_actor_ai(world: &World, actor: Entity) {
    world.add(actor, Ai);
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
    shooter: Option<Entity>,
) -> Entity {
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

pub fn set_state(world: &mut World, game_type: Option<GameType>) {
    let state = match game_type {
        Some(GameType::Server(..)) => State::Server,
        Some(GameType::Client(..)) => State::Client,
        None => State::None,
    };

    world.insert(state);
}
