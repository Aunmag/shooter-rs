pub struct MouseInput {
    pub delta_x: f32,
    pub delta_y: f32,
}

impl Default for MouseInput {
    fn default() -> Self {
        return Self {
            delta_x: 0.0,
            delta_y: 0.0,
        };
    }
}
