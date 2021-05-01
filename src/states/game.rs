use crate::components::Actor;
use crate::components::ActorActions;
use crate::components::RigidBody;
use crate::models::GameType;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use crate::resources::Message;
use crate::resources::MouseInput;
use crate::resources::NetResource;
use crate::states::ui::HomeState;
use crate::utils;
use crate::utils::Position;
use crate::utils::TakeContent;
use amethyst::controls::HideCursor;
use amethyst::core::transform::Transform;
use amethyst::ecs::Entity;
use amethyst::ecs::Join;
use amethyst::ecs::World;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::winit::DeviceEvent;
use amethyst::winit::ElementState;
use amethyst::winit::Event;
use amethyst::winit::MouseButton;
use amethyst::winit::VirtualKeyCode;
use amethyst::winit::WindowEvent;
use std::net::SocketAddr;

pub struct GameState {
    game_type: GameType,
    root: Option<Entity>,
}

impl GameState {
    pub fn new(game_type: GameType) -> Self {
        return Self {
            game_type,
            root: None,
        };
    }

    fn init_world_entities(&mut self, world: &mut World) {
        let root = world.create_entity().build();
        self.root.replace(root);

        if self.game_type.is_server() {
            let mut tasks = world.write_resource::<GameTaskResource>();
            let entity = world.entities().create();

            tasks.push(GameTask::ActorSpawn {
                entity,
                position: Position::default(),
            });

            tasks.push(GameTask::ActorGrant { entity });

            for i in 0..2 {
                let entity = world.entities().create();

                tasks.push(GameTask::ActorSpawn {
                    entity,
                    position: Position::new(5.0 * (0.5 - i as f32), 0.0, 0.0),
                });

                tasks.push(GameTask::ActorAiSet { entity });
            }
        }

        utils::world::create_terrain(world, root);
        utils::world_decorations::create_decorations(world, root);
    }

    fn on_task(&mut self, world: &mut World, task: GameTask) {
        match task {
            GameTask::Start => {
                // Skip since this should be processed while `ConnectingSate`
            }
            GameTask::ClientJoin(address) => {
                self.on_task_client_join(world, address);
            }
            GameTask::ActorSpawn { entity, position } => {
                self.on_task_actor_spawn(world, entity, position);
            }
            GameTask::ActorGrant { entity } => {
                self.on_task_actor_grant(world, entity);
            }
            GameTask::ActorAiSet { entity } => {
                self.on_task_actor_ai_set(world, entity);
            }
            GameTask::ActorAction {
                entity,
                actions,
                direction,
            } => {
                self.on_task_actor_action(world, entity, Some(actions), direction);
            }
            GameTask::ActorTurn { entity, direction } => {
                self.on_task_actor_action(world, entity, None, direction);
            }
            GameTask::ProjectileSpawn {
                position,
                velocity,
                acceleration_factor,
                shooter,
            } => {
                self.on_task_projectile_spawn(
                    world,
                    position,
                    velocity,
                    acceleration_factor,
                    shooter,
                );
            }
            GameTask::ProjectileHit {
                entity,
                force_x,
                force_y,
            } => {
                self.on_task_projectile_hit(world, entity, force_x, force_y);
            }
            GameTask::MessageSent {
                message,
                address_filter,
            } => {
                self.on_task_message_send(world, message, address_filter);
            }
        }
    }

    #[allow(clippy::unused_self)]
    fn on_task_client_join(&self, world: &mut World, address: SocketAddr) {
        let mut net = world.write_resource::<NetResource>();

        net.send_to(&address, Message::JoinAccept { id: 0 });

        for (entity, _, transform) in (
            &world.entities(),
            &world.read_storage::<Actor>(),
            &world.read_storage::<Transform>(),
        )
            .join()
        {
            net.send_to(
                &address,
                Message::ActorSpawn {
                    id: 0,
                    entity_id: entity.id(),
                    position: transform.into(),
                },
            );
        }

        let entity = world.entities().create();
        let mut tasks = world.write_resource::<GameTaskResource>();

        tasks.push(GameTask::ActorSpawn {
            entity,
            position: Position::default(),
        });

        tasks.push(GameTask::MessageSent {
            message: Message::ActorGrant {
                id: 0,
                entity_id: entity.id(),
            },
            address_filter: Some(address),
        });

        net.attach_entity(&address, entity);
    }

    fn on_task_actor_spawn(&self, world: &mut World, entity: Entity, position: Position) {
        if let Some(root) = self.root {
            utils::world::create_actor(world, root, entity, position, false, &self.game_type);

            if self.game_type.is_server() {
                world
                    .write_resource::<NetResource>()
                    .send_to_all(Message::ActorSpawn {
                        id: 0,
                        entity_id: entity.id(),
                        position,
                    });
            }
        }
    }

    fn on_task_actor_grant(&mut self, world: &mut World, entity: Entity) {
        if let Some(root) = self.root {
            utils::world::grant_played_actor(world, root, entity, &self.game_type);
        }
    }

    #[allow(clippy::unused_self)]
    fn on_task_actor_ai_set(&self, world: &World, entity: Entity) {
        utils::world::set_actor_ai(world, entity);
    }

    #[allow(clippy::unused_self)]
    fn on_task_actor_action(
        &self,
        world: &World,
        entity: Entity,
        actions: Option<ActorActions>,
        direction: f32,
    ) {
        if let Some(actions) = actions {
            if let Some(actor) = world.write_storage::<Actor>().get_mut(entity) {
                actor.actions = actions;
            }
        }

        if let Some(transform) = world.write_storage::<Transform>().get_mut(entity) {
            transform.set_rotation_2d(direction);
        }
    }

    fn on_task_projectile_spawn(
        &self,
        world: &mut World,
        position: Position,
        velocity: f32,
        acceleration_factor: f32,
        shooter: Option<Entity>,
    ) {
        if let Some(root) = self.root {
            if self.game_type.is_server() {
                world
                    .write_resource::<NetResource>()
                    .send_to_all(Message::ProjectileSpawn {
                        id: 0,
                        position,
                        velocity,
                        acceleration_factor,
                        shooter_id: shooter.map(Entity::id),
                    });
            }

            utils::world::create_projectile(
                world,
                root,
                position,
                velocity,
                acceleration_factor,
                shooter,
            );
        }
    }

    #[allow(clippy::unused_self)]
    fn on_task_projectile_hit(
        &self,
        world: &mut World,
        entity: Entity,
        force_x: f32,
        force_y: f32,
    ) {
        if let Some(body) = world.write_storage::<RigidBody>().get_mut(entity) {
            body.push(force_x, force_y, 0.0, true, false);
        }
    }

    #[allow(clippy::unused_self)]
    fn on_task_message_send(
        &self,
        world: &World,
        message: Message,
        address_filter: Option<SocketAddr>,
    ) {
        let mut net = world.write_resource::<NetResource>();

        if let Some(address) = address_filter {
            net.send_to(&address, message);
        } else {
            net.send_to_all(message);
        }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.init_world_entities(&mut data.world);
        utils::ui::set_cursor_visibility(&data.world, false);
        utils::world::set_state(&mut data.world, Some(self.game_type));
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        utils::world::set_state(&mut data.world, None);

        if let Some(root) = self.root.take() {
            if let Err(error) = data.world.delete_entity(root) {
                log::error!("Failed to delete the root entity: {}", error);
            }
        }
    }

    fn on_pause(&mut self, mut data: StateData<GameData>) {
        utils::world::set_state(&mut data.world, None);
    }

    fn on_resume(&mut self, mut data: StateData<GameData>) {
        utils::ui::set_cursor_visibility(&data.world, false);
        utils::world::set_state(&mut data.world, Some(self.game_type));
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        loop {
            let tasks = data
                .world
                .write_resource::<GameTaskResource>()
                .take_content();

            if tasks.is_empty() {
                break;
            }

            for task in tasks {
                self.on_task(&mut data.world, task);
            }
        }

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
