use crate::components::ActorActions;
use crate::components::ActorType;
use crate::utils::Position;
use amethyst::ecs::Entity;
use std::net::SocketAddr;

pub type GameTaskResource = Vec<GameTask>;

pub enum GameTask {
    Start,
    ClientJoin(SocketAddr),
    ActorSpawn {
        entity: Entity,
        actor_type: &'static ActorType,
        position: Position,
    },
    ActorGrant {
        entity: Entity,
    },
    ActorAction {
        entity: Entity,
        actions: ActorActions,
        direction: f32,
    },
    ActorTurn {
        entity: Entity,
        direction: f32,
    },
    ProjectileSpawn {
        position: Position,
        velocity: f32,
        acceleration_factor: f32,
        shooter: Option<Entity>,
    },
    ProjectileHit {
        entity: Entity,
        force_x: f32,
        force_y: f32,
    },
    EntityDelete(Entity),
}
