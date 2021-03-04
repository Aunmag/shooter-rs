use crate::components::ActorActions;
use crate::resources::Message;
use amethyst::ecs::Entity;
use std::net::SocketAddr;

pub type GameTaskResource = Vec<GameTask>;

pub enum GameTask {
    ClientGreet(SocketAddr),
    ActorSpawn {
        external_id: u16,
        x: f32,
        y: f32,
        direction: f32,
    },
    ActorGrant {
        external_id: u16,
    },
    ActorAiSet {
        external_id: u16,
    },
    ActorAction {
        external_id: u16,
        actions: ActorActions,
        direction: f32,
    },
    ActorTurn {
        external_id: u16,
        direction: f32,
    },
    TransformSync {
        external_id: u16,
        x: f32,
        y: f32,
        direction: f32,
    },
    ProjectileSpawn {
        x: f32,
        y: f32,
        velocity_x: f32,
        velocity_y: f32,
        acceleration_factor: f32,
        shooter_id: Option<u16>,
    },
    ProjectileHit {
        entity: Entity,
        force_x: f32,
        force_y: f32,
    },
    MessageSent {
        message: Message,
        address_filter: Option<SocketAddr>,
    },
}
