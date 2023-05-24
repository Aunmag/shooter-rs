use bevy::ecs::component::Component;

#[derive(Component)]
pub struct HealthBar {
    pub value: f32,
}

impl Default for HealthBar {
    fn default() -> Self {
        return Self { value: 1.0 };
    }
}
