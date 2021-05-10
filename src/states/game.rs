use crate::components::Actor;
use crate::components::ActorActions;
use crate::components::ActorType;
use crate::components::Health;
use crate::components::Projectile;
use crate::components::RigidBody;
use crate::models::GameType;
use crate::resources::EntityConverter;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use crate::resources::Message;
use crate::resources::MouseInput;
use crate::resources::NetResource;
use crate::states::ui::HomeState;
use crate::utils;
use crate::utils::Position;
use crate::utils::TakeContent;
use crate::utils::WorldExtCustom;
use amethyst::controls::HideCursor;
use amethyst::core::transform::Transform;
use amethyst::core::Time;
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
            let entity = world.entities().create();

            world.create_actor(
                root,
                entity,
                ActorType::HUMAN,
                Position::default(),
                false,
                &self.game_type,
            );

            world.set_actor_player(root, entity, &self.game_type);

            for i in 0..2 {
                let entity = world.entities().create();

                world.create_actor(
                    root,
                    entity,
                    ActorType::ZOMBIE,
                    Position::new(5.0 * (0.5 - i as f32), 0.0, 0.0),
                    false,
                    &self.game_type,
                );

                world.set_actor_ai(entity);
            }
        }

        world.create_terrain(root);
        utils::world_decorations::create_decorations(world, root);
    }

    fn on_task(&mut self, world: &mut World, task: &GameTask) {
        match *task {
            GameTask::Start => {
                // Skip since this should be processed while `ConnectingSate`
            }
            GameTask::ClientJoin(address) => {
                self.on_task_client_join(world, address);
            }
            GameTask::ActorSpawn {
                entity,
                actor_type,
                position,
            } => {
                self.on_task_actor_spawn(world, entity, actor_type, position);
            }
            GameTask::ActorGrant { entity } => {
                self.on_task_actor_grant(world, entity);
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
            GameTask::EntityDelete(entity) => {
                self.on_task_entity_delete(world, entity);
            }
        }
    }

    #[allow(clippy::unused_self)]
    fn on_task_client_join(&self, world: &mut World, address: SocketAddr) {
        {
            let mut net = world.write_resource::<NetResource>();

            net.send_to(&address, Message::JoinAccept { id: 0 });

            for (entity, actor, transform) in (
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
                        actor_type: actor.actor_type.serialized,
                        position: transform.into(),
                    },
                );
            }
        }

        if let Some(root) = self.root {
            let entity = world.entities().create();

            world.create_actor(
                root,
                entity,
                ActorType::HUMAN,
                Position::default(),
                false,
                &self.game_type,
            );

            let mut net = world.write_resource::<NetResource>();

            net.send_to(
                &address,
                Message::ActorGrant {
                    id: 0,
                    entity_id: entity.id(),
                },
            );

            net.attach_entity(&address, entity);
        }
    }

    fn on_task_actor_spawn(
        &self,
        world: &mut World,
        entity: Entity,
        actor_type: &'static ActorType,
        position: Position,
    ) {
        if let Some(root) = self.root {
            world.create_actor(root, entity, actor_type, position, false, &self.game_type);
        }
    }

    fn on_task_actor_grant(&mut self, world: &mut World, entity: Entity) {
        if let Some(root) = self.root {
            world.set_actor_player(root, entity, &self.game_type);
        }
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

            world.create_projectile(root, position, velocity, acceleration_factor, shooter);
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
            body.push(
                force_x * Projectile::PUSH_FACTOR,
                force_y * Projectile::PUSH_FACTOR,
                0.0,
                true,
                false,
            );
        }

        if let Some(health) = world.write_storage::<Health>().get_mut(entity) {
            health.damage(
                utils::math::length(force_x, force_y),
                world.read_resource::<Time>().absolute_time(),
            );
        }
    }

    #[allow(clippy::unused_self)]
    fn on_task_entity_delete(&self, world: &mut World, entity: Entity) {
        if self.game_type.is_server() {
            world
                .write_resource::<NetResource>()
                .send_to_all(Message::EntityDelete {
                    id: 0,
                    entity_id: entity.id(),
                });
        }

        world.write_resource::<EntityConverter>().remove(entity);

        if let Err(error) = world.delete_entity(entity) {
            log::error!("Failed to delete an entity: {}", error);
        }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.init_world_entities(&mut data.world);
        utils::ui::set_cursor_visibility(&data.world, false);
        data.world.set_state(Some(self.game_type));
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        data.world.set_state(None);

        if let Some(root) = self.root.take() {
            if let Err(error) = data.world.delete_entity(root) {
                log::error!("Failed to delete the root entity: {}", error);
            }
        }
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        data.world.set_state(None);
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        utils::ui::set_cursor_visibility(&data.world, false);
        data.world.set_state(Some(self.game_type));
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

            for task in tasks.iter() {
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
