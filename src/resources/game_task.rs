use std::net::SocketAddr;

pub type GameTaskResource = Vec<GameTask>;

pub enum GameTask {
    PlayerConnect(SocketAddr),
    ActorSpawn {
        public_id: u16,
        x: f32,
        y: f32,
        angle: f32,
    },
    ActorGrant(u16),
    ActorAction {
        public_id: u16,
        move_x: f32,
        move_y: f32,
        angle: f32,
    },
    TransformSync {
        public_id: u16,
        x: f32,
        y: f32,
        angle: f32,
    },
}
