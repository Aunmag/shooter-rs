use crate::components::Actor;
use crate::components::Player;
use crate::components::Terrain;
use crate::components::TransformSync;
use crate::data::LAYER_ACTOR;
use crate::data::LAYER_ACTOR_PLAYER;
use crate::data::LAYER_CAMERA;
use crate::input;
use crate::resources::ClientMessageResource;
use crate::resources::EntityIndexMap;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use crate::resources::ServerMessageResource;
use crate::states::menu::HomeState;
use crate::systems::CameraSystem;
use crate::systems::ClientSystem;
use crate::systems::InputSyncSystem;
use crate::systems::InterpolationSystem;
use crate::systems::PlayerSystem;
use crate::systems::ServerSystem;
use crate::systems::TerrainSystem;
use crate::systems::TransformSyncSystem;
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
use amethyst::renderer::resources::Tint;
use amethyst::renderer::Camera;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::Transparent;

use amethyst::renderer::palette::Srgba;
use amethyst::winit::DeviceEvent;
use amethyst::winit::Event;
use amethyst::winit::VirtualKeyCode;
use std::net::SocketAddr;
use std::sync::Arc;
use utils::math;

const MAX_TRANSFORM_DESYNC: f32 = 4.0; // TODO: Tweak

pub struct GameState<'a, 'b> {
    game_type: GameType,
    root: Option<Entity>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    ghost: Option<Entity>,
    temp_server: Option<ServerSystem>,
    temp_client: Option<ClientSystem>,
}

impl GameState<'_, '_> {
    pub fn new(game_type: GameType) -> Self {
        let temp_server;
        let temp_client;

        match game_type {
            GameType::Single => {
                temp_server = None;
                temp_client = None;
            }
            GameType::Join(address) => {
                temp_server = None;
                temp_client = Some(ClientSystem::new(address).unwrap());
            }
            GameType::Host(port) => {
                temp_server = Some(ServerSystem::new(port).unwrap());
                temp_client = None;
            }
        }

        return Self {
            game_type,
            root: None,
            dispatcher: None,
            ghost: None,
            temp_server,
            temp_client,
        };
    }

    fn create_dispatcher(&mut self, world: &mut World) {
        let mut builder = DispatcherBuilder::new();
        builder.add(CameraSystem::new(), "", &[]);
        builder.add(InputSyncSystem::new(), "", &[]);
        builder.add(InterpolationSystem, "", &[]);
        builder.add(PlayerSystem, "", &[]);
        builder.add(TerrainSystem, "", &[]);

        if let Some(server) = self.temp_server.take() {
            builder.add(server, "", &[]);
            builder.add(TransformSyncSystem::new(), "", &[]);
        }

        if let Some(client) = self.temp_client.take() {
            builder.add(client, "", &[]);
        }

        let mut dispatcher = builder
            .with_pool(Arc::clone(&world.read_resource::<ArcThreadPool>()))
            .build();

        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
    }

    fn process_tasks(&mut self, world: &mut World) {
        // TODO: Optimize iterations
        let mut tasks = Vec::new();

        if let Some(mut new_tasks) = world.try_fetch_mut::<GameTaskResource>() {
            tasks.append(&mut new_tasks);
        }

        for task in tasks.iter() {
            match *task {
                GameTask::ActorSpawn {
                    entity_id,
                    x,
                    y,
                    angle,
                } => {
                    if let Some(root) = self.root {
                        create_actor(world, entity_id, x, y, angle, false, root);
                    }
                }
                GameTask::ActorGrant(entity_id) => {
                    self.take_actor_grant(world, entity_id);
                }
                GameTask::ActorAction {
                    entity_id,
                    move_x,
                    move_y,
                    angle,
                } => {
                    self.perform_command(world, entity_id, move_x, move_y, angle);
                }
                GameTask::TransformSync {
                    entity_id,
                    x,
                    y,
                    angle,
                } => {
                    self.sync_transform(world, entity_id, x, y, angle);
                }
            }
        }
    }

    fn take_actor_grant(&mut self, world: &mut World, id: u16) {
        if let Some(entity) = fetch_entity_by_external_id(world, id) {
            world
                .write_storage::<Player>()
                .insert(entity, Player::new())
                .unwrap();

            if let Some(transform) = world.write_storage::<Transform>().get_mut(entity) {
                transform.set_translation_z(LAYER_ACTOR_PLAYER);
            }

            create_camera(world, entity);

            if let GameType::Join(..) = self.game_type {
                if let Some(ghost) = self.ghost.take() {
                    #[allow(unused_must_use)]
                    {
                        world.delete_entity(ghost);
                    }
                }

                if let Some(root) = self.root {
                    self.ghost.replace(
                        create_actor(world, 0, 0.0, 0.0, 0.0, true, root)
                    );
                }
            }
        }
    }

    fn perform_command(
        &self,
        world: &mut World,
        entity_id: u16,
        move_x: f32,
        move_y: f32,
        angle: f32,
    ) {
        if let Some(entity) = fetch_entity_by_external_id(world, entity_id) {
            let entity_to_update;

            if is_player(entity, world) {
                entity_to_update = self.ghost;
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

    fn sync_transform(&self, world: &mut World, id: u16, x: f32, y: f32, angle: f32) {
        if let Some(entity) = fetch_entity_by_external_id(world, id) {
            let is_player = is_player(entity, world);

            if let Some(transform) = world.write_storage::<TransformSync>().get_mut(entity) {
                if !is_player || !math::are_close(
                    x,
                    y,
                    transform.target_x,
                    transform.target_y,
                    MAX_TRANSFORM_DESYNC,
                ) {
                    transform.target_x = x;
                    transform.target_y = y;
                    transform.target_angle = angle;
                }
            }

            if is_player {
                if let Some(ghost) = self.ghost {
                    if let Some(transform) = world.write_storage::<TransformSync>().get_mut(ghost) {
                        transform.target_x = x;
                        transform.target_y = y;
                        transform.target_angle = angle;
                    }
                }
            }
        }
    }
}

impl<'a, 'b> SimpleState for GameState<'a, 'b> {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        data.world.register::<Actor>();
        let root = data.world.create_entity().build();
        self.root = Some(root);

        self.create_dispatcher(&mut data.world);

        Terrain::create_entity(data.world, root);

        data.world.insert(EntityIndexMap::new());
        data.world.insert(GameTaskResource::new());

        let create_player;

        match self.game_type {
            GameType::Single => {
                create_player = true;
            }
            GameType::Join(..) => {
                create_player = false;
                data.world.insert(Some(ClientMessageResource::new()));
            }
            GameType::Host(..) => {
                create_player = true;
                data.world.insert(Some(ServerMessageResource::new()));
            }
        }

        // TODO: Improve this
        if create_player {
            let entity_id = data.world.write_resource::<EntityIndexMap>().generate();
            create_actor(data.world, entity_id, 0.0, 0.0, 0.0, false, root);
            self.take_actor_grant(data.world, entity_id);
        }

        utils::set_cursor_visibility(false, &mut data.world);
        input::reset_mouse_delta();
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root) = self.root.take() {
            if let Err(error) = data.world.delete_entity(root) {
                log::error!("Failed to delete the root entity. Details: {}", error);
            }
        }
    }

    fn on_resume(&mut self, mut data: StateData<GameData>) {
        utils::set_cursor_visibility(false, &mut data.world);
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

fn create_actor(
    world: &mut World,
    entity_id: u16,
    x: f32,
    y: f32,
    angle: f32,
    is_ghost: bool,
    root: Entity,
) -> Entity {
    // TODO: Cache sprite render
    let renderer = SpriteRender::new(
        utils::load_sprite_sheet(world, "actors/human/image.png", "actors/human/image.ron"),
        0,
    );

    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, LAYER_ACTOR);
    transform.set_rotation_2d(angle);

    let mut actor_builder = world
        .create_entity()
        .with(renderer)
        .with(Actor::new())
        .with(transform)
        .with(TransformSync::new(x, y, angle))
        .with(Parent { entity: root });

    if is_ghost {
        actor_builder = actor_builder
            .with(Tint(Srgba::new(0.6, 0.6, 0.6, 0.4)))
            .with(Transparent);
    }

    let actor = actor_builder.build();

    if entity_id != 0 {
        if let Some(mut map) = world.try_fetch_mut::<EntityIndexMap>() {
            map.insert(actor.id(), entity_id);
        }
    }

    return actor;
}

fn fetch_entity_by_external_id(world: &World, id: u16) -> Option<Entity> {
    return world
        .try_fetch::<EntityIndexMap>()
        .and_then(|m| m.to_internal(id))
        .map(|id| world.entities().entity(id)); // TODO: What if entity doesn't exists
}

fn is_player(entity: Entity, world: &World) -> bool {
    // TODO: Optimize by caching player entity ID
    return world.read_storage::<Player>().get(entity).is_some();
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
