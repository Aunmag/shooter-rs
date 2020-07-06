use crate::components::actor::Actor;
use crate::components::player::Player;
use crate::utils;
use crate::ARENA_SIZE;
use amethyst::assets::AssetStorage;
use amethyst::assets::Loader;
use amethyst::core::math::Point3;
use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::core::Parent;
use amethyst::ecs::prelude::World;
use amethyst::ecs::Entity;
use amethyst::input::is_close_requested;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::renderer::sprite::SpriteSheetHandle;
use amethyst::renderer::Camera;
use amethyst::renderer::ImageFormat;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::SpriteSheet;
use amethyst::renderer::SpriteSheetFormat;
use amethyst::renderer::Texture;
use amethyst::tiles::MortonEncoder;
use amethyst::tiles::Tile;
use amethyst::tiles::TileMap;
use amethyst::winit::VirtualKeyCode;

#[derive(Default, Clone)]
pub struct ExampleTile;
impl Tile for ExampleTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        return Some(1);
    }
}

#[derive(Default)]
pub struct Game {}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data; // TODO: Learn more

        let actor_renderer = SpriteRender {
            // TODO: Simplify sprite loading, avoid using sprite sheets
            sprite_sheet: load_sprite_sheet(
                world,
                "actors/human/image.png",
                "actors/human/image.ron",
            ),
            sprite_number: 0,
        };

        create_actor(world, 50.0, 0.0, false, actor_renderer.clone());
        create_actor(world, -50.0, 0.0, false, actor_renderer.clone());
        create_actor(world, 0.0, 50.0, false, actor_renderer.clone());
        create_actor(world, 0.0, -50.0, false, actor_renderer.clone());

        let actor_main = create_actor(world, 0.0, 0.0, true, actor_renderer);

        create_camera(world, actor_main);
        create_ground(world);
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        return Trans::None;
    }
}

fn create_actor(
    world: &mut World,
    x: f32,
    y: f32,
    is_player: bool,
    renderer: SpriteRender,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.0);
    transform.set_rotation_2d(utils::math::PI_0_5);

    let mut actor = world
        .create_entity()
        .with(renderer)
        .with(Actor::new())
        .with(transform);

    if is_player {
        actor = actor.with(Player::new());
    }

    return actor.build();
}

fn create_camera(world: &mut World, player: Entity) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);
    transform.set_rotation_2d(utils::math::PI_1_5);

    return world
        .create_entity()
        .with(Camera::standard_2d(ARENA_SIZE * 1.5, ARENA_SIZE * 1.5))
        .with(transform)
        .with(Parent { entity: player })
        .build();
}

fn create_ground(world: &mut World) -> Entity {
    let map = TileMap::<ExampleTile, MortonEncoder>::new(
        Vector3::new(2, 2, 1), // how many
        Vector3::new(128, 128, 1), // size of one
        Some(load_sprite_sheet(
            world,
            "ground/grass.png",
            "ground/grass.ron",
        )),
    );

    return world
        .create_entity()
        .with(map)
        .with(Transform::default())
        .build();
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    return world.read_resource::<Loader>().load(
        ron_path,
        SpriteSheetFormat(world.read_resource::<Loader>().load(
            png_path,
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )),
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    );
}
