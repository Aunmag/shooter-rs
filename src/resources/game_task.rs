pub type GameTaskResource = Vec<GameTask>;

#[derive(Debug)]
pub enum GameTask {
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
