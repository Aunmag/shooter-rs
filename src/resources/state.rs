pub enum State {
    Any,
    Server,
    Client,
    None,
}

impl Default for State {
    fn default() -> Self {
        return Self::None;
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        return matches!(
            (self, other),
            // Straight
            (Self::None, Self::None)
            | (Self::Server, Self::Server)
            | (Self::Client, Self::Client)
            // Any vs Server
            | (Self::Any, Self::Server)
            | (Self::Server, Self::Any)
            // Any vs Client
            | (Self::Any, Self::Client)
            | (Self::Client, Self::Any)
        );
    }
}
