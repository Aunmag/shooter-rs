use crate::model::SpriteOffset;
use bevy::ecs::component::Component;
use enumset::EnumSet;
use enumset::EnumSetType;
use serde::Deserialize;
use serde::Serialize;

#[derive(Component)]
pub struct Actor {
    pub config: &'static ActorConfig,
    pub actions: EnumSet<ActorAction>,
    pub look_at: f32,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActorType {
    Human,
    Zombie,
}

pub struct ActorConfig {
    pub sprite: &'static str,
    pub sprite_offset: SpriteOffset,
    pub movement_velocity: f32,
    pub rotation_velocity: f32,
    pub sprint_factor: f32,
    pub resistance: f32,
    pub radius: f32,
    pub mass: f32,
    pub actor_type: ActorType,
}

#[derive(Debug, Serialize, Deserialize, EnumSetType)]
pub enum ActorAction {
    MovementForward,
    MovementBackward,
    MovementLeftward,
    MovementRightward,
    Sprint,
    Attack,
}

impl Actor {
    pub const fn new(config: &'static ActorConfig) -> Self {
        return Self {
            config,
            actions: EnumSet::EMPTY,
            look_at: 0.0,
        };
    }
}

impl ActorConfig {
    pub const HUMAN: &'static Self = &Self {
        sprite: "actors/human/image.png",
        sprite_offset: SpriteOffset::new(None, Some(9.0)),
        movement_velocity: 2.5,
        rotation_velocity: 8.0,
        sprint_factor: 2.0,
        resistance: 8000.0,
        radius: 0.25,
        mass: 80_000.0,
        actor_type: ActorType::Human,
    };

    pub const ZOMBIE: &'static Self = &Self {
        sprite: "actors/zombie/image.png",
        sprite_offset: SpriteOffset::new(None, Some(6.5)),
        movement_velocity: Self::HUMAN.movement_velocity * 0.4,
        rotation_velocity: Self::HUMAN.rotation_velocity * 0.4,
        sprint_factor: 1.8,
        resistance: Self::HUMAN.resistance * 0.4,
        radius: 0.21,
        mass: 70_000.0,
        actor_type: ActorType::Zombie,
    };
}

impl From<ActorType> for &'static ActorConfig {
    fn from(actor_type: ActorType) -> Self {
        return match actor_type {
            ActorType::Human => ActorConfig::HUMAN,
            ActorType::Zombie => ActorConfig::ZOMBIE,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_set_bit_width() {
        assert_eq!(EnumSet::<ActorAction>::bit_width(), 6);
    }

    #[test]
    fn test_action_set_empty() {
        let actions = EnumSet::<ActorAction>::EMPTY;
        assert_eq!(actions.len(), 0);
        assert_eq!(actions.as_u8(), 0b0);
        assert_eq!(actions.as_u8(), 0);
    }

    #[test]
    fn test_action_set_full() {
        let actions = EnumSet::<ActorAction>::all();
        assert_eq!(actions.len(), 6);
        assert_eq!(actions.as_u8(), 0b111111);
        assert_eq!(actions.as_u8(), 63);
    }

    #[test]
    fn test_action_set_complex() {
        let actions = ActorAction::MovementLeftward | ActorAction::Sprint;
        assert_eq!(actions.len(), 2);
        assert_eq!(actions.as_u8(), 0b10100);
        assert_eq!(actions.as_u8(), 20);
    }
}
