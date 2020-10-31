pub struct GameStatus {
    pub is_loaded: bool,
}

impl Default for GameStatus {
    fn default() -> Self {
        return Self { is_loaded: false };
    }
}
