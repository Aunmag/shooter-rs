use crate::components::actor::Actor;
use crate::components::player::Player;
use crate::states::menu::home::Home;
use crate::utils;
use crate::ARENA_SIZE;
use amethyst::assets::AssetStorage;
use amethyst::assets::Loader;
use amethyst::core::math::Point3;
use amethyst::core::math::Vector3;
use amethyst::core::shrev::EventChannel;
use amethyst::core::transform::Transform;
use amethyst::core::Parent;
use amethyst::ecs::prelude::World;
use amethyst::ecs::Entity;
use amethyst::input::is_key_down;
use amethyst::input::InputEvent;
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

#[derive(Debug)]
pub enum GameEvent {
    GameStart,
    GameEnd,
}

pub struct Game {
    root: Option<Entity>,
}

impl Game {
    pub fn new() -> Self {
        return Self { root: None };
    }
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;

        utils::set_cursor_visibility(false, &mut world);

        let actor_renderer = SpriteRender {
            // TODO: Simplify sprite loading, avoid using sprite sheets
            sprite_sheet: load_sprite_sheet(
                world,
                "actors/human/image.png",
                "actors/human/image.ron",
            ),
            sprite_number: 0,
        };

        let root = world.create_entity().build();

        create_actor(
            world,
            50.0,
            0.0,
            false,
            actor_renderer.clone(),
            root.clone(),
        );

        create_actor(
            world,
            -50.0,
            0.0,
            false,
            actor_renderer.clone(),
            root.clone(),
        );

        create_actor(
            world,
            0.0,
            50.0,
            false,
            actor_renderer.clone(),
            root.clone(),
        );

        create_actor(
            world,
            0.0,
            -50.0,
            false,
            actor_renderer.clone(),
            root.clone(),
        );

        let actor_main = create_actor(world, 0.0, 0.0, true, actor_renderer, root.clone());

        create_camera(world, actor_main);
        create_ground(world, root.clone());

        utils::input::reset_mouse_delta();

        world
            .write_resource::<EventChannel<GameEvent>>()
            .single_write(GameEvent::GameStart);

        self.root = Some(root);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root) = self.root.take() {
            // TODO: Do not panic, write warning to the log instead
            data.world
                .delete_entity(root)
                .expect("Failed to delete the root entity. Was it already removed?");
        }

        data.world
            .write_resource::<EventChannel<GameEvent>>()
            .single_write(GameEvent::GameEnd);
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        let StateData { mut world, .. } = data;
        utils::set_cursor_visibility(false, &mut world);
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Push(Box::new(Home::new(false)));
                }
            }
            StateEvent::Input(event) => {
                if let InputEvent::MouseMoved {
                    delta_x: delta,
                    delta_y: _,
                } = event {
                    utils::input::add_mouse_delta(delta as i16);
                }
            }
            _ => {}
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
    root: Entity,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.0);
    transform.set_rotation_2d(utils::math::PI_0_5);

    let mut actor = world
        .create_entity()
        .with(renderer)
        .with(Actor::new())
        .with(transform)
        .with(Parent { entity: root });

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

fn create_ground(world: &mut World, root: Entity) -> Entity {
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
        .with(Parent { entity: root })
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
