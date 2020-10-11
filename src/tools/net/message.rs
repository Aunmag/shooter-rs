use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

pub trait Message: Sized + Serialize + DeserializeOwned + Debug {
    fn encode(&self) -> Vec<u8> {
        return bincode::serialize(self).unwrap();
    }

    fn decode(encoded: &[u8]) -> Result<Self, bincode::Error> {
        return bincode::deserialize_from(encoded);
    }

    fn into_response(id: u16) -> Self;

    fn set_id(&mut self, id_new: u16);

    fn has_id(&self) -> bool;

    fn get_id(&self) -> Option<u16>;
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ServerMessage {
    Response {
        message_id: u16,
    },
    ActorSpawn {
        id: u16,
        entity_id: u16,
        x: f32,
        y: f32,
        angle: f32,
    },
    ActorGrant {
        id: u16,
        entity_id: u16,
    },
    ActorAction {
        id: u16,
        entity_id: u16,
        move_x: f32,
        move_y: f32,
        angle: f32,
    },
    TransformSync {
        id: u16,
        entity_id: u16,
        x: f32,
        y: f32,
        angle: f32,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ClientMessage {
    Response {
        message_id: u16,
    },
    Greeting {
        id: u16,
    },
    ActorAction {
        id: u16,
        move_x: f32,
        move_y: f32,
        angle: f32,
    },
}

impl Message for ServerMessage {
    fn into_response(id: u16) -> Self {
        return ServerMessage::Response { message_id: id };
    }

    fn set_id(&mut self, id_new: u16) {
        #[allow(clippy::match_same_arms)]
        match *self {
            Self::Response { .. } => {}
            Self::ActorSpawn { ref mut id, .. } => {
                *id = id_new;
            }
            Self::ActorGrant { ref mut id, .. } => {
                *id = id_new;
            }
            Self::ActorAction { ref mut id, .. } => {
                *id = id_new;
            }
            Self::TransformSync { ref mut id, .. } => {
                *id = id_new;
            }
        }
    }

    fn has_id(&self) -> bool {
        #[allow(clippy::match_same_arms)]
        return match *self {
            Self::Response { .. } => false,
            Self::ActorSpawn { .. } => true,
            Self::ActorGrant { .. } => true,
            Self::ActorAction { .. } => true,
            Self::TransformSync { .. } => true,
        };
    }

    fn get_id(&self) -> Option<u16> {
        #[allow(clippy::match_same_arms)]
        return match *self {
            Self::Response { .. } => None,
            Self::ActorSpawn { id, .. } => Some(id),
            Self::ActorGrant { id, .. } => Some(id),
            Self::ActorAction { id, .. } => Some(id),
            Self::TransformSync { id, .. } => Some(id),
        };
    }
}

impl Message for ClientMessage {
    fn into_response(id: u16) -> Self {
        return ClientMessage::Response { message_id: id };
    }

    fn set_id(&mut self, id_new: u16) {
        #[allow(clippy::match_same_arms)]
        match *self {
            Self::Response { .. } => {}
            Self::Greeting { ref mut id } => {
                *id = id_new;
            }
            Self::ActorAction { ref mut id, .. } => {
                *id = id_new;
            }
        }
    }

    fn has_id(&self) -> bool {
        #[allow(clippy::match_same_arms)]
        return match *self {
            Self::Response { .. } => false,
            Self::Greeting { .. } => true,
            Self::ActorAction { .. } => true,
        };
    }

    fn get_id(&self) -> Option<u16> {
        #[allow(clippy::match_same_arms)]
        return match *self {
            Self::Response { .. } => None,
            Self::Greeting { id } => Some(id),
            Self::ActorAction { id, .. } => Some(id),
        };
    }
}

pub enum MessageContainer<T: Message> {
    Decoded(T), // TODO: Find a better name
    Encoded(Vec<u8>),
}
