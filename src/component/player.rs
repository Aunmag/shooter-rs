use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component)]
pub struct Player {
    pub is_aiming: bool,
    pub is_controllable: bool,
    pub crosshair: PlayerCrosshair,
    extra_rotation: f32,
}

impl Player {
    pub const EXTRA_ROTATION_MULTIPLAYER: f32 = 0.1;
    pub const EXTRA_ROTATION_MAX: f32 = 0.11;

    pub fn new(is_controllable: bool, crosshair: Entity) -> Self {
        return Self {
            is_aiming: false,
            is_controllable,
            crosshair: PlayerCrosshair::new(crosshair),
            extra_rotation: 0.0,
        };
    }

    pub fn add_extra_rotation(&mut self, value: f32) -> f32 {
        let previous = self.extra_rotation;
        let limit = Self::EXTRA_ROTATION_MAX;
        self.extra_rotation = (self.extra_rotation + value).clamp(-limit, limit);
        let added = self.extra_rotation - previous;
        return added;
    }

    pub fn get_extra_rotation(&self) -> f32 {
        return self.extra_rotation;
    }
}

pub struct PlayerCrosshair {
    pub entity: Entity,
    pub distance: f32,
}

impl PlayerCrosshair {
    pub fn new(entity: Entity) -> Self {
        return Self {
            entity,
            distance: 1.0,
        };
    }
}
