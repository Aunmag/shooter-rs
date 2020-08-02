use crate::components::actor::Actor;
use crate::components::player::Player;
use crate::input;
use crate::states::menu::home::Home;
use crate::systems::player::PlayerSystem;
use crate::utils;
use amethyst::assets::AssetStorage;
use amethyst::assets::Loader;
use amethyst::core::math::Point3;
use amethyst::core::math::Vector3;
use amethyst::core::shrev::EventChannel;
use amethyst::core::transform::Transform;
use amethyst::core::ArcThreadPool;
use amethyst::core::Parent;
use amethyst::ecs::prelude::World;
use amethyst::ecs::Dispatcher;
use amethyst::ecs::DispatcherBuilder;
use amethyst::ecs::Entity;
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
use amethyst::winit::DeviceEvent;
use amethyst::winit::Event;
use amethyst::winit::VirtualKeyCode;
use std::sync::Arc;

const VIEWPORT: f32 = 150.0; // TODO: Do not hard-code

#[derive(Debug)]
pub enum GameEvent {
    GameStart,
    GameEnd,
}

pub struct Game<'a, 'b> {
    root: Option<Entity>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl Game<'_, '_> {
    pub fn new() -> Self {
        return Self {
            root: None,
            dispatcher: None,
        };
    }

    fn create_dispatcher(&mut self, world: &mut World) {
        let mut builder = DispatcherBuilder::new();
        builder.add(PlayerSystem, "", &[]);

        let mut dispatcher = builder
            .with_pool(Arc::clone(&world.read_resource::<ArcThreadPool>()))
            .build();

        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
    }
}

impl<'a, 'b> SimpleState for Game<'a, 'b> {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.create_dispatcher(&mut data.world);
        utils::set_cursor_visibility(false, &mut data.world);

        let actor_renderer = SpriteRender {
            // TODO: Simplify sprite loading, avoid using sprite sheets
            sprite_sheet: load_sprite_sheet(
                data.world,
                "actors/human/image.png",
                "actors/human/image.ron",
            ),
            sprite_number: 0,
        };

        let root = data.world.create_entity().build();

        create_actor(data.world, 50.0, 0.0, false, actor_renderer.clone(), root);
        create_actor(data.world, -50.0, 0.0, false, actor_renderer.clone(), root);
        create_actor(data.world, 0.0, 50.0, false, actor_renderer.clone(), root);
        create_actor(data.world, 0.0, -50.0, false, actor_renderer.clone(), root);

        let actor_main = create_actor(data.world, 0.0, 0.0, true, actor_renderer, root);

        create_camera(data.world, actor_main);
        create_ground(data.world, root);

        input::reset_mouse_delta();

        data.world
            .write_resource::<EventChannel<GameEvent>>()
            .single_write(GameEvent::GameStart);

        self.root = Some(root);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root) = self.root.take() {
            data.world
                .delete_entity(root)
                .expect("Failed to delete the root entity. Was it already removed?");
        }

        data.world
            .write_resource::<EventChannel<GameEvent>>()
            .single_write(GameEvent::GameEnd);
    }

    fn on_resume(&mut self, mut data: StateData<GameData>) {
        utils::set_cursor_visibility(false, &mut data.world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(data.world);
        }

        return Trans::None;
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if let Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } = event {
                #[allow(clippy::cast_possible_truncation)]
                input::add_mouse_delta(delta.0 as i16);
            }

            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(Home::new(false)));
            }
        }

        return Trans::None;
    }
}

#[derive(Default, Clone)]
pub struct GroundTile;

impl Tile for GroundTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        return Some(1);
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

    return world
        .create_entity()
        .with(Camera::standard_2d(VIEWPORT, VIEWPORT))
        .with(transform)
        .with(Parent { entity: player })
        .build();
}

fn create_ground(world: &mut World, root: Entity) -> Entity {
    let map = TileMap::<GroundTile, MortonEncoder>::new(
        Vector3::new(2, 2, 1),
        Vector3::new(128, 128, 1),
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
