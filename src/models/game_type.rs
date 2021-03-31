use std::net::SocketAddr;

#[derive(Clone, Copy)]
pub enum GameType {
    Server(u16),
    Client(SocketAddr),
}

impl GameType {
    pub fn is_server(&self) -> bool {
        return match *self {
            Self::Server(..) => true,
            Self::Client(..) => false,
        };
    }
}
