use crate::components::Actor;
use crate::components::TransformSync;
use crate::input;
use crate::resources::ClientMessageResource;
use crate::resources::EntityIndexMap;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use crate::resources::ServerMessageResource;
use crate::states::ui::HomeState;
use crate::systems::CameraSystem;
use crate::systems::ClientSystem;
use crate::systems::InputSyncSystem;
use crate::systems::InterpolationSystem;
use crate::systems::PlayerSystem;
use crate::systems::ServerSystem;
use crate::systems::TerrainSystem;
use crate::systems::TransformSyncSystem;
use crate::utils;
use amethyst::core::ArcThreadPool;
use amethyst::ecs::prelude::World;
use amethyst::ecs::Dispatcher;
use amethyst::ecs::DispatcherBuilder;
use amethyst::ecs::Entity;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::winit::DeviceEvent;
use amethyst::winit::Event;
use amethyst::winit::VirtualKeyCode;
use std::net::SocketAddr;
use std::sync::Arc;
use utils::math;

const MAX_TRANSFORM_SYNC_OFFSET: f32 = 4.0; // TODO: Tweak

pub struct GameState<'a, 'b> {
    game_type: GameType,
    root: Option<Entity>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    player_actor: Option<Entity>,
    player_ghost: Option<Entity>,
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
        builder.add(CameraSystem::new(), "", &[]);
        builder.add(InputSyncSystem::new(), "", &[]);
        builder.add(InterpolationSystem, "", &[]);
        builder.add(PlayerSystem, "", &[]);
        builder.add(TerrainSystem, "", &[]);

        match self.game_type {
            GameType::Single => {}
            GameType::Join(address) => {
                builder.add(ClientSystem::new(address).unwrap(), "", &[]);
                builder.add(TransformSyncSystem::new(), "", &[]);
            }
            GameType::Host(port) => {
                builder.add(ServerSystem::new(port).unwrap(), "", &[]);
            }
        }

        let mut dispatcher = builder
            .with_pool(Arc::clone(&world.read_resource::<ArcThreadPool>()))
            .build();

        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
    }

    fn init_resources(&self, world: &mut World) {
        world.register::<Actor>();
        world.insert(EntityIndexMap::new());
        world.insert(GameTaskResource::new());

        match self.game_type {
            GameType::Single => {}
            GameType::Join(..) => {
                world.insert(Some(ClientMessageResource::new()));
            }
            GameType::Host(..) => {
                world.insert(Some(ServerMessageResource::new()));
            }
        }
    }

    fn init_world_entities(&mut self, world: &mut World) {
        let root = world.create_entity().build();
        self.root.replace(root);

        if self.is_own_game() {
            let public_id = world.write_resource::<EntityIndexMap>().generate();
            utils::world::create_actor(world, root, public_id, 0.0, 0.0, 0.0, false);
            self.on_task_actor_grant(world, public_id); // TODO: Do not call `on_task_actor_grant`
        }

        utils::world::create_terrain(world, root);
    }

    fn process_tasks(&mut self, world: &mut World) {
        let mut tasks = Vec::with_capacity(0);

        {
            // TODO: Move to tasks resource
            let mut new_tasks = world.fetch_mut::<GameTaskResource>();
            tasks.reserve_exact(new_tasks.capacity());
            std::mem::swap(&mut tasks, &mut new_tasks);
        }

        for task in tasks.drain(..) {
            match task {
                GameTask::ActorSpawn {
                    entity_id,
                    x,
                    y,
                    angle,
                } => {
                    self.on_task_actor_spawn(world, entity_id, x, y, angle);
                }
                GameTask::ActorGrant(entity_id) => {
                    self.on_task_actor_grant(world, entity_id);
                }
                GameTask::ActorAction {
                    entity_id,
                    move_x,
                    move_y,
                    angle,
                } => {
                    self.on_task_actor_action(world, entity_id, move_x, move_y, angle);
                }
                GameTask::TransformSync {
                    entity_id,
                    x,
                    y,
                    angle,
                } => {
                    self.on_task_transform_sync(world, entity_id, x, y, angle);
                }
            }
        }
    }

    fn on_task_actor_spawn(&self, world: &mut World, entity_id: u16, x: f32, y: f32, angle: f32) {
        if let Some(root) = self.root {
            utils::world::create_actor(world, root, entity_id, x, y, angle, false);
        }
    }

    fn on_task_actor_grant(&mut self, world: &mut World, id: u16) {
        if let Some(root) = self.root {
            if let Some(actor) = EntityIndexMap::fetch_entity_by_external_id(world, id) {
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
        entity_id: u16,
        move_x: f32,
        move_y: f32,
        angle: f32,
    ) {
        if let Some(entity) = EntityIndexMap::fetch_entity_by_external_id(world, entity_id) {
            let entity_to_update;

            if self.is_player_actor(entity) {
                entity_to_update = self.player_ghost;
            } else {
                entity_to_update = Some(entity);
            }

            if let Some(entity) = entity_to_update {
                if let Some(transform) = world.write_storage::<TransformSync>().get_mut(entity) {
                    transform.target_x += move_x * Actor::MOVEMENT_VELOCITY;
                    transform.target_y += move_y * Actor::MOVEMENT_VELOCITY;
                    transform.target_angle = angle;
                }
            }
        }
    }

    fn on_task_transform_sync(&self, world: &mut World, id: u16, x: f32, y: f32, angle: f32) {
        if let Some(entity) = EntityIndexMap::fetch_entity_by_external_id(world, id) {
            let is_player = self.is_player_actor(entity);

            if let Some(transform) = world.write_storage::<TransformSync>().get_mut(entity) {
                if !is_player || !math::are_close(
                    x,
                    y,
                    transform.target_x,
                    transform.target_y,
                    MAX_TRANSFORM_SYNC_OFFSET,
                ) {
                    transform.target_x = x;
                    transform.target_y = y;
                    transform.target_angle = angle;
                }
            }

            if is_player {
                if let Some(ghost) = self.player_ghost {
                    if let Some(transform) = world.write_storage::<TransformSync>().get_mut(ghost) {
                        transform.target_x = x;
                        transform.target_y = y;
                        transform.target_angle = angle;
                    }
                }
            }
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
        input::reset_mouse_delta();
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

        self.process_tasks(data.world);

        return Trans::None;
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans {
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

pub enum GameType {
    Single,
    Join(SocketAddr),
    Host(u16),
}
