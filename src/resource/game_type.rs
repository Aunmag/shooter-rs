use crate::model::Arguments;

#[derive(Clone, Copy, Debug)]
pub enum GameType {
    Server,
    Client,
}

impl GameType {
    pub const fn is_server(self) -> bool {
        return matches!(self, Self::Server);
    }

    pub const fn is_client(&self) -> bool {
        return matches!(self, Self::Client);
    }
}

impl TryFrom<&Arguments> for GameType {
    type Error = ();

    fn try_from(arguments: &Arguments) -> Result<Self, Self::Error> {
        return match (arguments.host, arguments.join) {
            (true, true) => Err(()),
            (false, false) => Ok(GameType::Server),
            (true, false) => Ok(GameType::Server),
            (false, true) => Ok(GameType::Client),
        };
    }
}
