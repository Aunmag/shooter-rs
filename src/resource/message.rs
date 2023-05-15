use crate::component::ActorAction;
use crate::component::ActorType;
use crate::model::Position;
use bincode::Options;
use enumset::EnumSet;
use serde::Deserialize;
use serde::Serialize;

// TODO: move to model

pub const MESSAGE_SIZE_MAX: usize = std::mem::size_of::<Message>();

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Message {
    Response {
        message_id: u16,
    },
    Join {
        id: u16,
    },
    JoinAccept {
        id: u16,
    },
    ClientInput {
        id: u16,
        actions: EnumSet<ActorAction>,
        direction: f32,
    },
    ClientInputDirection {
        id: u16,
        direction: f32,
    },
    ActorSpawn {
        id: u16,
        entity_index: u32,
        actor_type: ActorType,
        position: Position,
    },
    ActorGrant {
        id: u16,
        entity_index: u32,
    },
    PositionUpdate {
        entity_index: u32,
        position: Position,
    },
    ProjectileSpawn {
        id: u16,
        position: Position,
        velocity: f32,
        acceleration_factor: f32,
        shooter_id: Option<u32>,
    },
    EntityDelete {
        id: u16,
        entity_index: u32,
    },
}

impl Message {
    pub fn encode(&self) -> Vec<u8> {
        #[allow(clippy::unwrap_used)] // it's ok to unwrap here
        return bincode::DefaultOptions::new().serialize(self).unwrap();
    }

    pub fn decode(encoded: &[u8]) -> Result<Self, bincode::Error> {
        return bincode::DefaultOptions::new().deserialize(encoded);
    }

    pub fn set_id(&mut self, id_new: u16) {
        #[allow(clippy::match_same_arms)]
        match *self {
            Self::Response { .. } => {}
            Self::Join { ref mut id } => {
                *id = id_new;
            }
            Self::JoinAccept { ref mut id } => {
                *id = id_new;
            }
            Self::ClientInput { ref mut id, .. } => {
                *id = id_new;
            }
            Self::ClientInputDirection { ref mut id, .. } => {
                *id = id_new;
            }
            Self::ActorSpawn { ref mut id, .. } => {
                *id = id_new;
            }
            Self::ActorGrant { ref mut id, .. } => {
                *id = id_new;
            }
            Self::PositionUpdate { .. } => {}
            Self::ProjectileSpawn { ref mut id, .. } => {
                *id = id_new;
            }
            Self::EntityDelete { ref mut id, .. } => {
                *id = id_new;
            }
        }
    }

    pub const fn get_id(&self) -> Option<u16> {
        #[allow(clippy::match_same_arms)]
        return match *self {
            Self::Response { .. } => None,
            Self::Join { id } => Some(id),
            Self::JoinAccept { id } => Some(id),
            Self::ClientInput { id, .. } => Some(id),
            Self::ClientInputDirection { id, .. } => Some(id),
            Self::ActorSpawn { id, .. } => Some(id),
            Self::ActorGrant { id, .. } => Some(id),
            Self::PositionUpdate { .. } => None,
            Self::ProjectileSpawn { id, .. } => Some(id),
            Self::EntityDelete { id, .. } => Some(id),
        };
    }

    pub const fn has_id(&self) -> bool {
        return !matches!(self, Self::Response { .. });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_with_actor_actions() {
        let initial = Message::ClientInput {
            id: 250,
            actions: ActorAction::MovementLeftward | ActorAction::Sprint,
            direction: 11.5625,
        };

        let encoded = initial.encode();
        assert_eq!(initial, Message::decode(&encoded).unwrap());
        assert_eq!(encoded.len(), 7);
    }
}
