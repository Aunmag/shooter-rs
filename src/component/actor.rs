use crate::model::SpriteOffset;
use bevy::ecs::component::Component;
use serde::Deserialize;
use serde::Serialize;

#[derive(Component)]
pub struct Actor {
    pub config: &'static ActorConfig,
    pub actions: ActorActions,
    pub rotation: f32,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ActorType {
    Human,
    Zombie,
}

pub struct ActorConfig {
    pub sprite: &'static str,
    pub sprite_offset: SpriteOffset,
    pub movement_velocity: f32,
    pub resistance: f32,
    pub radius: f32,
    pub mass: f32,
    pub actor_type: ActorType,
}

// TODO: try use enum set
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
    pub const fn new(config: &'static ActorConfig) -> Self {
        return Self {
            config,
            actions: ActorActions::empty(),
            rotation: 0.0,
        };
    }
}

impl ActorConfig {
    pub const HUMAN: &'static Self = &Self {
        sprite: "actors/human/image.png",
        sprite_offset: SpriteOffset::new(None, Some(9.0)),
        movement_velocity: 2.0,
        resistance: 8000.0,
        radius: 0.25,
        mass: 80_000.0,
        actor_type: ActorType::Human,
    };

    pub const ZOMBIE: &'static Self = &Self {
        sprite: "actors/zombie/image.png",
        sprite_offset: SpriteOffset::new(None, Some(6.5)),
        movement_velocity: Self::HUMAN.movement_velocity * 0.4,
        resistance: Self::HUMAN.resistance * 0.4,
        radius: 0.21,
        mass: 70_000.0,
        actor_type: ActorType::Zombie,
    };
}

impl Into<&'static ActorConfig> for ActorType {
    fn into(self) -> &'static ActorConfig {
        return match self {
            Self::Human => ActorConfig::HUMAN,
            Self::Zombie => ActorConfig::ZOMBIE,
        };
    }
}

impl Default for ActorActions {
    fn default() -> Self {
        return Self::empty();
    }
}
