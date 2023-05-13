use bevy::ecs::schedule::States;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Game,
}
