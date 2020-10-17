mod entity_index_map;
mod game_task;
mod ui_task;

pub use self::entity_index_map::*;
pub use self::game_task::*;
pub use self::ui_task::*;

use crate::tools::net::message::ClientMessage;
use crate::tools::net::message::ServerMessage;

pub type ClientMessageResource = Vec<ClientMessage>;
pub type ServerMessageResource = Vec<ServerMessage>;
