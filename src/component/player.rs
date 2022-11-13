use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;

#[derive(Component)]
pub struct Player {
    pub ghost: Option<Entity>,
}

impl Player {
    pub const fn new(ghost: Option<Entity>) -> Self {
        return Self { ghost };
    }
}
