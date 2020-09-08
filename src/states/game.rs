use crate::components::Actor;
use crate::components::Player;
use crate::components::Terrain;
use crate::data::LAYER_ACTOR;
use crate::data::LAYER_ACTOR_PLAYER;
use crate::data::LAYER_CAMERA;
use crate::input;
use crate::resources::UiTask;
use crate::resources::UiTaskResource;
use crate::states::menu;
use crate::states::menu::HomeState;
use crate::systems::CameraSystem;
use crate::systems::PlayerSystem;
use crate::systems::TerrainSystem;
use crate::utils;
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

pub struct GameState<'a, 'b> {
    root: Option<Entity>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl GameState<'_, '_> {
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

    fn set_buttons_availability(data: &mut StateData<GameData>, is_availability: bool) {
        let mut tasks = data.world.write_resource::<UiTaskResource>();

        tasks.push(UiTask::SetButtonAvailability(
            menu::home::BUTTON_CONTINUE_ID,
            is_availability,
        ));

        tasks.push(UiTask::SetButtonAvailability(
            menu::quit::BUTTON_DISCONNECT_ID,
            is_availability,
        ));
    }
}

impl<'a, 'b> SimpleState for GameState<'a, 'b> {
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

        Self::set_buttons_availability(&mut data, true);

        self.root = Some(root);
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        if let Some(root) = self.root.take() {
            if let Err(error) = data.world.delete_entity(root) {
                log::error!("Failed to delete the root entity. Details: {}", error);
            }
        }

        Self::set_buttons_availability(&mut data, false);
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
                return Trans::Push(Box::new(HomeState::new(false)));
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
    let z;

    if is_player {
        z = LAYER_ACTOR_PLAYER;
    } else {
        z = LAYER_ACTOR;
    }

    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, z);

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
    transform.set_translation_xyz(0.0, 0.0, LAYER_CAMERA);

    return world
        .create_entity()
        .with(Camera::standard_2d(1.0, 1.0))
        .with(transform)
        .with(Parent { entity: player })
        .build();
}
