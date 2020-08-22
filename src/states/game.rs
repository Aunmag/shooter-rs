use crate::components::actor::Actor;
use crate::components::player::Player;
use crate::components::terrain::Terrain;
use crate::input;
use crate::states::menu::home::Home;
use crate::systems::camera::CameraSystem;
use crate::systems::player::PlayerSystem;
use crate::systems::terrain::TerrainSystem;
use crate::utils;
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
use amethyst::renderer::Camera;
use amethyst::renderer::SpriteRender;
use amethyst::winit::DeviceEvent;
use amethyst::winit::Event;
use amethyst::winit::VirtualKeyCode;
use std::sync::Arc;

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
        builder.add(CameraSystem::new(), "", &[]);
        builder.add(PlayerSystem, "", &[]);
        builder.add(TerrainSystem, "", &[]); // TODO: Maybe run while fixed update

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

        let actor_renderer = SpriteRender::new(
            utils::load_sprite_sheet(
                data.world,
                "actors/human/image.png",
                "actors/human/image.ron",
            ),
            0,
        );

        let root = data.world.create_entity().build();

        create_actor(data.world, 50.0, 0.0, false, actor_renderer.clone(), root);
        create_actor(data.world, -50.0, 0.0, false, actor_renderer.clone(), root);
        create_actor(data.world, 0.0, 50.0, false, actor_renderer.clone(), root);
        create_actor(data.world, 0.0, -50.0, false, actor_renderer.clone(), root);

        let actor_main = create_actor(data.world, 0.0, 0.0, true, actor_renderer, root);

        create_camera(data.world, actor_main);
        Terrain::create_entity(data.world, root);

        input::reset_mouse_delta();

        data.world
            .write_resource::<EventChannel<GameEvent>>()
            .single_write(GameEvent::GameStart);

        self.root = Some(root);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root) = self.root.take() {
            if let Err(error) = data.world.delete_entity(root) {
                log::error!("Failed to delete the root entity. Details: {}", error);
            }
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
        .with(Camera::standard_2d(1.0, 1.0))
        .with(transform)
        .with(Parent { entity: player })
        .build();
}
