use crate::utils::math;
use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;

pub struct Player {
    pub accumulated_input: AccumulatedInput,
}

pub struct AccumulatedInput {
    movement_x: f32,
    movement_y: f32,
    rotation: f32,
}

impl Player {
    pub fn new() -> Self {
        return Self {
            accumulated_input: AccumulatedInput::new(),
        };
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl AccumulatedInput {
    pub fn new() -> Self {
        return Self {
            movement_x: 0.0,
            movement_y: 0.0,
            rotation: 0.0,
        };
    }

    pub fn prepend(&mut self, movement_x: f32, movement_y: f32, rotation: f32) {
        self.movement_x += movement_x;
        self.movement_y += movement_y;
        self.rotation += rotation;
    }

    pub fn take(&mut self) -> (f32, f32, f32) {
        let movement_x = self.movement_x;
        let movement_y = self.movement_y;
        let rotation = math::normalize_radians(self.rotation);

        self.movement_x = 0.0;
        self.movement_y = 0.0;
        self.rotation = 0.0;

        return (movement_x, movement_y, rotation);
    }
}
