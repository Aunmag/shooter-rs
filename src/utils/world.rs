use crate::components::Actor;
use crate::components::Player;
use crate::components::Terrain;
use crate::components::TransformSync;
use crate::data::LAYER_ACTOR;
use crate::data::LAYER_ACTOR_PLAYER;
use crate::data::LAYER_CAMERA;
use crate::data::LAYER_TERRAIN;
use crate::resources::EntityIndexMap;
use crate::utils;
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
    public_id: u16,
    x: f32,
    y: f32,
    angle: f32,
    is_ghost: bool,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, LAYER_ACTOR);
    transform.set_rotation_2d(angle);

    // TODO: Cache renderer
    let renderer = SpriteRender::new(
        utils::load_sprite_sheet(world, "actors/human/image.png", "actors/human/image.ron"),
        0,
    );

    let mut builder = world
        .create_entity()
        .with(Parent { entity: root })
        .with(Actor)
        .with(transform)
        .with(TransformSync::new(x, y, angle))
        .with(renderer);

    if is_ghost {
        builder = builder
            .with(Tint(Srgba::new(0.6, 0.6, 0.6, 0.4)))
            .with(Transparent);
    }

    let actor = builder.build();

    if public_id != 0 {
        world
            .fetch_mut::<EntityIndexMap>()
            .insert(actor.id(), public_id);
    }

    return actor;
}

pub fn grant_played_actor(
    world: &mut World,
    root: Entity,
    actor: Entity,
    create_ghost: bool,
) -> Option<Entity> {
    // TODO: Remove old player entity
    // TODO: Reset layer for old transform
    // TODO: Remove old ghost

    world
        .write_storage::<Player>()
        .insert(actor, Player::new())
        .unwrap(); // TODO: No unwrap

    if let Some(transform) = world.write_storage::<Transform>().get_mut(actor) {
        transform.set_translation_z(LAYER_ACTOR_PLAYER);
    }

    create_camera(world, actor);

    if create_ghost {
        // TODO: Maybe make ghost as player's child
        return Some(create_actor(world, root, 0, 0.0, 0.0, 0.0, true));
    } else {
        return None;
    }
}

pub fn create_camera(world: &mut World, player: Entity) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, LAYER_CAMERA);

    return world
        .create_entity()
        .with(Camera::standard_2d(1.0, 1.0))
        .with(transform)
        .with(Parent { entity: player })
        .build();
}

pub fn create_terrain(world: &mut World, root: Entity) -> Entity {
    let tile_map = TileMap::<Terrain, MortonEncoder>::new(
        Vector3::new(Terrain::QUANTITY, Terrain::QUANTITY, 1),
        Vector3::new(Terrain::SIZE, Terrain::SIZE, 1),
        Some(utils::load_sprite_sheet(
            world,
            "ground/grass.png",
            "ground/grass.ron",
        )),
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
