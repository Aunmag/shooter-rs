use crate::components::Actor;
use crate::components::Interpolation;
use crate::components::Player;
use crate::components::Terrain;
use crate::components::TransformSync;
use crate::data::LAYER_ACTOR;
use crate::data::LAYER_ACTOR_PLAYER;
use crate::data::LAYER_CAMERA;
use crate::data::LAYER_TERRAIN;
use crate::resources::EntityIndexMap;
use crate::resources::Sprite;
use crate::resources::SpriteResource;
use crate::states::GameType;
use amethyst::core::math::Vector3;
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
// TODO: Maybe don't use `EntityIndexMap`

pub fn create_actor(
    world: &mut World,
    root: Entity,
    public_id: Option<u16>,
    x: f32,
    y: f32,
    direction: f32,
    is_ghost: bool,
    game_type: &GameType,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, LAYER_ACTOR);
    transform.set_rotation_2d(direction);

    let mut renderer = world
        .read_resource::<SpriteResource>()
        .get(&Sprite::Actor)
        .map(|s| SpriteRender::new(s, 0));

    let mut builder = world
        .create_entity()
        .with(Parent { entity: root })
        .with(Actor::new())
        .with(transform);

    match *game_type {
        GameType::Single => {}
        GameType::Join(..) => {
            builder = builder.with(Interpolation::new());
        }
        GameType::Host(..) => {
            builder = builder.with(Interpolation::new());
            builder = builder.with(TransformSync);
        }
    }

    if let Some(renderer) = renderer.take() {
        builder = builder.with(renderer);
    }

    if is_ghost {
        builder = builder.with(Tint(Srgba::new(0.6, 0.6, 0.6, 0.4)));
        builder = builder.with(Transparent);
    }

    let actor = builder.build();

    if let Some(public_id) = public_id {
        world
            .fetch_mut::<EntityIndexMap>()
            .store(actor.id(), public_id);
    }

    return actor;
}

#[allow(clippy::unwrap_used)] // TODO: Remove
pub fn grant_played_actor(
    world: &mut World,
    root: Entity,
    actor: Entity,
    game_type: &GameType,
) -> Option<Entity> {
    // TODO: Remove old player entity
    // TODO: Reset layer for old transform
    // TODO: Remove old ghost

    world
        .write_storage::<Player>()
        .insert(actor, Player)
        .unwrap(); // TODO: No unwrap

    if let Some(transform) = world.write_storage::<Transform>().get_mut(actor) {
        transform.set_translation_z(LAYER_ACTOR_PLAYER);
    }

    create_camera(world, actor);

    if let GameType::Join(..) = *game_type {
        // TODO: Maybe make ghost as player's child
        return Some(create_actor(
            world, root, None, 0.0, 0.0, 0.0, true, game_type,
        ));
    } else {
        return None;
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
    let tile_map = TileMap::<Terrain, MortonEncoder>::new(
        Vector3::new(Terrain::QUANTITY, Terrain::QUANTITY, 1),
        Vector3::new(Terrain::SIZE, Terrain::SIZE, 1),
        world.read_resource::<SpriteResource>().get(&Sprite::Grass),
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
