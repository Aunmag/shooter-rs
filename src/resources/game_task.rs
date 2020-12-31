use crate::components::ActorActions;
use std::net::SocketAddr;

pub type GameTaskResource = Vec<GameTask>;

pub enum GameTask {
    PlayerConnect(SocketAddr),
    ActorSpawn {
        public_id: u16,
        x: f32,
        y: f32,
        direction: f32,
    },
    ActorGrant(u16),
    ActorAction {
        public_id: u16,
        actions: ActorActions,
        direction: f32,
    },
    ActorTurn {
        public_id: u16,
        direction: f32,
    },
    TransformSync {
        public_id: u16,
        x: f32,
        y: f32,
        direction: f32,
    },
}
