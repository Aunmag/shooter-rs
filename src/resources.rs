use crate::tools::net::message::ClientMessage;
use crate::tools::net::message::ServerMessage;
use bimap::BiMap;
use std::collections::HashMap;

pub type GameTaskResource = Vec<GameTask>;
pub type UiTaskResource = HashMap<String, UiTask>;
pub type ClientMessageResource = Vec<ClientMessage>;
pub type ServerMessageResource = Vec<ServerMessage>;

#[derive(Debug)]
pub enum GameTask {
    ActorSpawn {
        entity_id: u16,
        x: f32,
        y: f32,
        angle: f32,
    },
    ActorGrant(u16),
    ActorAction {
        entity_id: u16,
        move_x: f32,
        move_y: f32,
        angle: f32,
    },
    TransformSync {
        entity_id: u16,
        x: f32,
        y: f32,
        angle: f32,
    },
}

pub enum UiTask {
    SetButtonAvailability(bool),
    SetText(&'static str),
}

pub struct EntityIndexMap {
    map: BiMap<u32, u16>,
    last_generated_id: u16,
}

impl EntityIndexMap {
    pub fn new() -> Self {
        return Self {
            map: BiMap::new(),
            last_generated_id: 0,
        };
    }

    pub fn insert(&mut self, id_internal: u32, id_external: u16) {
        self.map.insert(id_internal, id_external);
    }

    // TODO: Make sure it call from server only
    pub fn generate(&mut self) -> u16 {
        self.last_generated_id = self.last_generated_id.wrapping_add(1);
        return self.last_generated_id;
    }

    pub fn to_internal(&self, id: u16) -> Option<u32> {
        return self.map.get_by_right(&id).copied();
    }

    pub fn to_external(&self, id: u32) -> Option<u16> {
        return self.map.get_by_left(&id).copied();
    }

    // TODO: Make sure it call from server only
    #[allow(clippy::wrong_self_convention)]
    pub fn to_external_or_generate(&mut self, id: u32) -> u16 {
        if let Some(id_external) = self.to_external(id) {
            return id_external;
        } else {
            let id_external = self.generate();
            self.insert(id, id_external);
            return id_external;
        }
    }
}

impl Default for EntityIndexMap {
    fn default() -> Self {
        return Self::new();
    }
}
