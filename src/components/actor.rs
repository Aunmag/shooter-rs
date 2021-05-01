use crate::resources::Sprite;
use amethyst::ecs::Component;
use amethyst::ecs::VecStorage;
use serde::Deserialize;
use serde::Serialize;

pub struct Actor {
    pub actor_type: &'static ActorType,
    pub actions: ActorActions,
    pub rotation: f32,
}

pub struct ActorType {
    pub sprite: Sprite,
    pub movement_velocity: f32,
    pub resistance: f32,
    pub radius: f32,
    pub mass: f32,
    pub serialized: ActorTypeSerialized,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ActorTypeSerialized {
    Human,
    Zombie,
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
    pub fn new(actor_type: &'static ActorType) -> Self {
        return Actor {
            actor_type,
            actions: ActorActions::empty(),
            rotation: 0.0,
        };
    }
}

impl Component for Actor {
    type Storage = VecStorage<Self>;
}

impl ActorType {
    pub const HUMAN: &'static Self = &Self {
        sprite: Sprite::ActorHuman,
        movement_velocity: 2.0,
        resistance: 8000.0,
        radius: 0.25,
        mass: 80_000.0,
        serialized: ActorTypeSerialized::Human,
    };

    pub const ZOMBIE: &'static Self = &Self {
        sprite: Sprite::ActorZombie,
        movement_velocity: Self::HUMAN.movement_velocity * 0.4,
        resistance: Self::HUMAN.resistance * 0.4,
        radius: 0.21,
        mass: 70_000.0,
        serialized: ActorTypeSerialized::Zombie,
    };
}

impl Into<&'static ActorType> for ActorTypeSerialized {
    fn into(self) -> &'static ActorType {
        return match self {
            ActorTypeSerialized::Human => ActorType::HUMAN,
            ActorTypeSerialized::Zombie => ActorType::ZOMBIE,
        };
    }
}
