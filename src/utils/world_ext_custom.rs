use crate::components::Actor;
use crate::components::ActorType;
use crate::components::Ai;
use crate::components::Collision;
use crate::components::Health;
use crate::components::Interpolation;
use crate::components::Own;
use crate::components::Player;
use crate::components::Projectile;
use crate::components::ProjectileConfig;
use crate::components::RigidBody;
use crate::components::Terrain;
use crate::components::Weapon;
use crate::components::WeaponConfig;
use crate::data::LAYER_ACTOR;
use crate::data::LAYER_ACTOR_PLAYER;
use crate::data::LAYER_CAMERA;
use crate::data::LAYER_TERRAIN;
use crate::models::GameType;
use crate::resources::Message;
use crate::resources::NetResource;
use crate::resources::Sprite;
use crate::resources::SpriteResource;
use crate::resources::State;
use crate::utils::Position;
use amethyst::core::math::Vector2;
use amethyst::core::math::Vector3;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::core::Parent;
use amethyst::ecs::Component;
use amethyst::ecs::Entity;
use amethyst::ecs::World;
use amethyst::ecs::WorldExt;
use amethyst::prelude::*;
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::Camera;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::Transparent;
use amethyst::tiles::MortonEncoder;
use amethyst::tiles::TileMap;

pub trait WorldExtCustom {
    fn add<T: Component>(&self, entity: Entity, component: T);

    fn create_simple_sprite(
        &mut self,
        root: Entity,
        x: f32,
        y: f32,
        z: f32,
        direction: f32,
        sprite: SpriteRender,
    ) -> Entity;

    fn create_actor(
        &self,
        root: Entity,
        entity: Entity,
        actor_type: &'static ActorType,
        position: Position,
        is_ghost: bool,
        game_type: &GameType,
    );

    fn create_camera(&mut self, target: Entity) -> Entity;

    fn create_projectile(
        &mut self,
        root: Entity,
        position: Position,
        velocity: f32,
        acceleration_factor: f32,
        shooter: Option<Entity>,
    ) -> Entity;

    fn create_terrain(&mut self, root: Entity) -> Entity;

    fn set_actor_player(&mut self, root: Entity, actor: Entity, game_type: &GameType);

    fn set_actor_ai(&self, actor: Entity);

    fn set_state(&mut self, game_type: Option<GameType>);
}

impl WorldExtCustom for World {
    fn add<T: Component>(&self, entity: Entity, component: T) {
        if let Err(error) = self.write_storage::<T>().insert(entity, component) {
            log::error!(
                "Failed to insert a component for Entity({}): {}",
                entity.id(),
                error,
            );
        }
    }

    fn create_simple_sprite(
        &mut self,
        root: Entity,
        x: f32,
        y: f32,
        z: f32,
        direction: f32,
        sprite: SpriteRender,
    ) -> Entity {
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, z);
        transform.set_rotation_2d(direction);

        return self
            .create_entity()
            .with(Parent { entity: root })
            .with(transform)
            .with(sprite)
            .build();
    }

    // TODO: Send message here
    fn create_actor(
        &self,
        root: Entity,
        entity: Entity,
        actor_type: &'static ActorType,
        position: Position,
        is_ghost: bool,
        game_type: &GameType,
    ) {
        if game_type.is_server() {
            self.write_resource::<NetResource>()
                .send_to_all(Message::ActorSpawn {
                    id: 0,
                    entity_id: entity.id(),
                    actor_type: actor_type.serialized,
                    position,
                });
        }

        let mut transform = Transform::default();
        transform.set_translation_xyz(position.x, position.y, LAYER_ACTOR);
        transform.set_rotation_2d(position.direction);

        self.add(entity, transform);
        self.add(entity, Parent { entity: root });
        self.add(entity, Actor::new(actor_type));
        self.add(
            entity,
            Weapon::new(WeaponConfig {
                muzzle_velocity: 320.0,
                fire_rate: 650.0,
                projectile: ProjectileConfig {
                    acceleration_factor: -7.0,
                },
            }),
        );

        match *game_type {
            GameType::Server(..) => {
                self.add(entity, Own);
                self.add(entity, Health::new(actor_type.resistance));
            }
            GameType::Client(..) => {
                let now = self.read_resource::<Time>().absolute_time();
                self.add(entity, Interpolation::new(position, now));
            }
        }

        if let Some(renderer) = self
            .read_resource::<SpriteResource>()
            .get(actor_type.sprite)
            .map(|s| SpriteRender::new(s, 0))
        {
            self.add(entity, renderer);
        }

        if is_ghost {
            self.add(entity, Tint(Srgba::new(0.6, 0.6, 0.6, 0.4)));
            self.add(entity, Transparent);
        } else {
            self.add(entity, RigidBody::new(actor_type.mass, 7.0, 8.0, 0.05));
            self.add(
                entity,
                Collision {
                    radius: actor_type.radius,
                },
            );
        }
    }

    fn create_camera(&mut self, target: Entity) -> Entity {
        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 0.0, LAYER_CAMERA);

        return self
            .create_entity()
            .with(Camera::standard_2d(1.0, 1.0))
            .with(transform)
            .with(Parent { entity: target })
            .build();
    }

    fn create_projectile(
        &mut self,
        root: Entity,
        position: Position,
        velocity: f32,
        acceleration_factor: f32,
        shooter: Option<Entity>,
    ) -> Entity {
        let (sin, cos) = (-position.direction).sin_cos();
        let projectile = Projectile::new(
            ProjectileConfig {
                acceleration_factor,
            },
            self.read_resource::<Time>().absolute_time(),
            Vector2::new(position.x, position.y),
            Vector2::new(velocity * sin, velocity * cos),
            shooter,
        );

        return self
            .create_entity()
            .with(Parent { entity: root })
            .with(projectile)
            .build();
    }

    fn create_terrain(&mut self, root: Entity) -> Entity {
        let quantity;

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        {
            quantity = Terrain::QUANTITY.abs().ceil() as u32;
        }

        let tile_map = TileMap::<Terrain, MortonEncoder>::new(
            Vector3::new(quantity, quantity, 1),
            Vector3::new(Terrain::SIZE, Terrain::SIZE, 1),
            self.read_resource::<SpriteResource>().get(Sprite::Grass),
        );

        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 0.0, LAYER_TERRAIN);

        return self
            .create_entity()
            .with(Parent { entity: root })
            .with(tile_map)
            .with(transform)
            .build();
    }

    fn set_actor_player(&mut self, root: Entity, actor: Entity, game_type: &GameType) {
        // TODO: Remove old player entity
        // TODO: Reset layer for old transform
        // TODO: Remove old ghost
        // TODO: Remove old camera
        // TODO: Remove old ownership
        // TODO: Maybe make ghost as player's child

        let ghost;

        match *game_type {
            GameType::Server(..) => {
                ghost = None;
            }
            GameType::Client(..) => {
                let entity = self.entities().create();

                self.create_actor(
                    root,
                    entity,
                    ActorType::HUMAN,
                    Position::default(),
                    true,
                    game_type,
                );

                ghost = Some(entity);
            }
        }

        self.add(actor, Own);
        self.add(actor, Player::new(ghost));

        if let Some(transform) = self.write_storage::<Transform>().get_mut(actor) {
            transform.set_translation_z(LAYER_ACTOR_PLAYER);
        }

        self.create_camera(actor);
    }

    fn set_actor_ai(&self, actor: Entity) {
        self.add(actor, Ai);
    }

    fn set_state(&mut self, game_type: Option<GameType>) {
        let state = match game_type {
            Some(GameType::Server(..)) => State::Server,
            Some(GameType::Client(..)) => State::Client,
            None => State::None,
        };

        self.insert(state);
    }
}
