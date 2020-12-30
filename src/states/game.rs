use crate::components::Actor;
use crate::components::Interpolation;
use crate::resources::EntityIndexMap;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use crate::resources::Message;
use crate::resources::MessageReceiver;
use crate::resources::MessageResource;
use crate::resources::MouseInput;
use crate::resources::NetworkTask;
use crate::resources::NetworkTaskResource;
use crate::states::ui::HomeState;
use crate::systems::net::InputSyncSystem;
use crate::systems::net::InterpolationSystem;
use crate::systems::net::NetworkSystem;
use crate::systems::net::TransformSyncSystem;
use crate::systems::CameraSystem;
use crate::systems::PlayerSystem;
use crate::systems::TerrainSystem;
use crate::utils;
use crate::utils::TakeContent;
use amethyst::controls::HideCursor;
use amethyst::core::transform::Transform;
use amethyst::core::ArcThreadPool;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::World;
use amethyst::ecs::Dispatcher;
use amethyst::ecs::DispatcherBuilder;
use amethyst::ecs::Entity;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::winit::DeviceEvent;
use amethyst::winit::ElementState;
use amethyst::winit::Event;
use amethyst::winit::MouseButton;
use amethyst::winit::VirtualKeyCode;
use amethyst::winit::WindowEvent;
use std::f32::consts::TAU;
use std::net::SocketAddr;
use std::sync::Arc;

const MAX_PLAYER_OFFSET: f32 = 0.25;

pub struct GameState<'a, 'b> {
    game_type: GameType,
    root: Option<Entity>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    player_actor: Option<Entity>,
    player_ghost: Option<Entity>,
}

pub enum GameType {
    Single,
    Join(SocketAddr),
    Host(u16),
}

impl GameState<'_, '_> {
    pub fn new(game_type: GameType) -> Self {
        return Self {
            game_type,
            root: None,
            dispatcher: None,
            player_actor: None,
            player_ghost: None,
        };
    }

    fn init_dispatcher(&mut self, world: &mut World) {
        let mut builder = DispatcherBuilder::new();
        builder.add(PlayerSystem, "Player", &[]);
        builder.add(CameraSystem::new(), "Camera", &[]);
        builder.add(TerrainSystem, "Terrain", &[]);

        #[allow(clippy::unwrap_used)] // TODO: Remove
        match self.game_type {
            GameType::Single => {}
            GameType::Join(address) => {
                builder.add(InputSyncSystem::new(), "InputSync", &["Player"]);
                builder.add(
                    NetworkSystem::new_as_client(address).unwrap(),
                    "Network",
                    &["InputSync"],
                );
                builder.add(InterpolationSystem, "Interpolation", &[]);
            }
            GameType::Host(port) => {
                builder.add(InputSyncSystem::new(), "InputSync", &["Player"]);
                builder.add(
                    NetworkSystem::new_as_server(port).unwrap(),
                    "Network",
                    &["InputSync"],
                );
                builder.add(InterpolationSystem, "Interpolation", &[]);
                builder.add(TransformSyncSystem::new(), "TransformSync", &[]);
            }
        }

        let mut dispatcher = builder
            .with_pool(Arc::clone(&world.read_resource::<ArcThreadPool>()))
            .build();

        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
    }

    #[allow(clippy::unused_self)]
    fn init_resources(&self, world: &mut World) {
        world.register::<Actor>();
        world.register::<Interpolation>();
        world.insert(EntityIndexMap::new());
        world.insert(GameTaskResource::new());
        world.insert(MessageResource::new());
        world.insert(NetworkTaskResource::new());
    }

    fn init_world_entities(&mut self, world: &mut World) {
        let root = world.create_entity().build();
        self.root.replace(root);

        if self.is_own_game() {
            let mut tasks = world.write_resource::<GameTaskResource>();
            let public_id = world
                .write_resource::<EntityIndexMap>()
                .generate_public_id();

            tasks.push(GameTask::ActorSpawn {
                public_id,
                x: 0.0,
                y: 0.0,
                angle: 0.0,
            });

            tasks.push(GameTask::ActorGrant(public_id));
        }

        utils::world::create_terrain(world, root);
    }

    #[allow(clippy::unused_self)]
    fn reset_input(&self, world: &World) {
        let mut mouse_input = world.write_resource::<MouseInput>();
        mouse_input.delta_x = 0.0;
        mouse_input.delta_y = 0.0;
    }

    fn sync_transform(&self, world: &mut World, entity: Entity, x: f32, y: f32) {
        let is_player = self.is_player_actor(entity);

        if let (Some(transform), Some(interpolation)) = (
            world.read_storage::<Transform>().get(entity),
            world.write_storage::<Interpolation>().get_mut(entity),
        ) {
            let offset_x = x - (transform.translation().x + interpolation.offset_x);
            let offset_y = y - (transform.translation().y + interpolation.offset_y);

            if !is_player || !utils::math::are_closer_than(
                offset_x,
                offset_y,
                0.0,
                0.0,
                MAX_PLAYER_OFFSET,
            ) {
                interpolation.offset_x += offset_x;
                interpolation.offset_y += offset_y;
            }
        }

        if is_player {
            if let Some(ghost) = self.player_ghost {
                self.sync_transform(world, ghost, x, y);
            }
        }
    }

    fn on_task_player_connect(&self, world: &mut World, address: SocketAddr) {
        let root = match self.root {
            Some(root) => root,
            None => return,
        };

        let public_id = world
            .write_resource::<EntityIndexMap>()
            .generate_public_id();

        utils::world::create_actor(world, root, Some(public_id), 0.0, 0.0, 0.0, false);

        let mut messages = world.write_resource::<MessageResource>();

        messages.push((
            MessageReceiver::Except(address),
            Message::ActorSpawn {
                id: 0,
                public_id,
                x: 0.0,
                y: 0.0,
                angle: 0.0,
            },
        ));

        world
            .write_resource::<NetworkTaskResource>()
            .push(NetworkTask::AttachPublicId(address, public_id));

        let id_map = world.read_resource::<EntityIndexMap>();

        for (entity, transform, interpolation, _) in (
            &world.entities(),
            &world.read_storage::<Transform>(),
            &world.read_storage::<Interpolation>(),
            &world.read_storage::<Actor>(),
        )
            .join()
        {
            if let Some(public_id) = id_map.to_public_id(entity.id()) {
                messages.push((
                    MessageReceiver::Only(address),
                    Message::ActorSpawn {
                        id: 0,
                        public_id,
                        x: transform.translation().x + interpolation.offset_x,
                        y: transform.translation().y + interpolation.offset_y,
                        angle: (transform.euler_angles().2 + interpolation.offset_angle) % TAU,
                    },
                ));
            }
        }

        messages.push((
            MessageReceiver::Only(address),
            Message::ActorGrant { id: 0, public_id },
        ));
    }

    fn on_task_actor_spawn(&self, world: &mut World, public_id: u16, x: f32, y: f32, angle: f32) {
        if let Some(root) = self.root {
            utils::world::create_actor(world, root, Some(public_id), x, y, angle, false);
        }
    }

    fn on_task_actor_grant(&mut self, world: &mut World, id: u16) {
        if let Some(root) = self.root {
            if let Some(actor) = EntityIndexMap::fetch_entity_by_public_id(world, id) {
                self.player_actor = Some(actor);
                self.player_ghost = utils::world::grant_played_actor(
                    world,
                    root,
                    actor,
                    !self.is_own_game(),
                );
            }
        }
    }

    fn on_task_actor_action(
        &self,
        world: &mut World,
        public_id: u16,
        move_x: f32,
        move_y: f32,
        angle: f32,
    ) {
        if let Some(entity) = EntityIndexMap::fetch_entity_by_public_id(world, public_id) {
            let entity_to_update;

            if self.is_player_actor(entity) {
                entity_to_update = self.player_ghost;
            } else {
                entity_to_update = Some(entity);
            }

            if let Some(entity) = entity_to_update {
                let mut interpolations = world.write_storage::<Interpolation>();

                if let Some(interpolation) = interpolations.get_mut(entity) {
                    interpolation.offset_x += move_x * Actor::MOVEMENT_VELOCITY;
                    interpolation.offset_y += move_y * Actor::MOVEMENT_VELOCITY;
                    interpolation.offset_angle = world
                        .read_storage::<Transform>()
                        .get(entity)
                        .map_or(
                            0.0,
                            |t| utils::math::get_radians_difference(angle, t.euler_angles().2),
                        );
                }
            }
        }
    }

    fn on_task_transform_sync(&self, world: &mut World, id: u16, x: f32, y: f32) {
        if let Some(entity) = EntityIndexMap::fetch_entity_by_public_id(world, id) {
            self.sync_transform(world, entity, x, y);
        }
    }

    fn is_player_actor(&self, entity: Entity) -> bool {
        return self.player_actor == Some(entity);
    }

    fn is_own_game(&self) -> bool {
        #[allow(clippy::match_same_arms)]
        return match self.game_type {
            GameType::Single => true,
            GameType::Join(..) => false,
            GameType::Host(..) => true,
        };
    }
}

impl<'a, 'b> SimpleState for GameState<'a, 'b> {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.init_dispatcher(&mut data.world);
        self.init_resources(&mut data.world);
        self.init_world_entities(&mut data.world);
        self.reset_input(&data.world);
        utils::ui::set_cursor_visibility(&mut data.world, false);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root) = self.root.take() {
            if let Err(error) = data.world.delete_entity(root) {
                log::error!("Failed to delete the root entity. Details: {}", error);
            }
        }
    }

    fn on_resume(&mut self, mut data: StateData<GameData>) {
        utils::ui::set_cursor_visibility(&mut data.world, false);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(data.world);
        }

        let mut tasks = data.world.fetch_mut::<GameTaskResource>().take_content();

        for task in tasks.drain(..) {
            match task {
                GameTask::PlayerConnect(address) => {
                    self.on_task_player_connect(data.world, address);
                }
                GameTask::ActorSpawn {
                    public_id,
                    x,
                    y,
                    angle,
                } => {
                    self.on_task_actor_spawn(data.world, public_id, x, y, angle);
                }
                GameTask::ActorGrant(public_id) => {
                    self.on_task_actor_grant(data.world, public_id);
                }
                GameTask::ActorAction {
                    public_id,
                    move_x,
                    move_y,
                    angle,
                } => {
                    self.on_task_actor_action(data.world, public_id, move_x, move_y, angle);
                }
                GameTask::TransformSync { public_id, x, y } => {
                    self.on_task_transform_sync(data.world, public_id, x, y);
                }
            }
        }

        self.reset_input(&data.world);

        return Trans::None;
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            let mut cursor = data.world.write_resource::<HideCursor>();

            match event {
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => {
                    if cursor.hide {
                        let mut mouse_input = data.world.write_resource::<MouseInput>();

                        #[allow(clippy::cast_possible_truncation)]
                        {
                            mouse_input.delta_x += delta.0 as f32;
                            mouse_input.delta_y += delta.1 as f32;
                        }
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
                } => {
                    cursor.hide = true;
                }
                _ => {}
            }

            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(HomeState::new(false)));
            }

            if cursor.hide && is_key_down(&event, VirtualKeyCode::Tab) {
                cursor.hide = false;
            }
        }

        return Trans::None;
    }
}
