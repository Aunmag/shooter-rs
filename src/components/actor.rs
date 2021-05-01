use amethyst::ecs::Component;
use amethyst::ecs::VecStorage;

pub struct Actor {
    pub actions: ActorActions,
    pub rotation: f32,
}

bitflags::bitflags! {
    pub struct ActorActions: u8 {
        const MOVEMENT_FORWARD   = 0b0000_0001;
        const MOVEMENT_BACKWARD  = 0b0000_0010;
        const MOVEMENT_LEFTWARD  = 0b0000_0100;
        const MOVEMENT_RIGHTWARD = 0b0000_1000;
        const ATTACK             = 0b0001_0000;
    }
}

impl Actor {
    pub const MOVEMENT_VELOCITY: f32 = 2.0;
    pub const RESISTANCE: f32 = 8000.0;

    pub fn new() -> Self {
        return Actor {
            actions: ActorActions::empty(),
            rotation: 0.0,
        };
    }
}

impl Component for Actor {
    type Storage = VecStorage<Self>;
}
