use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::ReadExpect;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::WriteStorage;
use amethyst::renderer::Camera;
use amethyst::window::ScreenDimensions;

const VIEW_DISTANCE: f32 = 180.0;
const OFFSET_RATIO: f32 = 0.25;

#[derive(SystemDesc)]
pub struct CameraSystem {
    screen_size_x: f32,
    screen_size_y: f32,
}

impl CameraSystem {
    pub fn new() -> Self {
        return Self {
            screen_size_x: 0.0,
            screen_size_y: 0.0,
        };
    }
}

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        ReadExpect<'a, ScreenDimensions>,
        WriteStorage<'a, Camera>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (screen, mut cameras, mut transforms): Self::SystemData) {
        let screen_size_x = screen.width();
        let screen_size_y = screen.height();

        #[allow(clippy::float_cmp)]
        if screen_size_x != self.screen_size_x || screen_size_y != self.screen_size_y {
            let scale = VIEW_DISTANCE / to_view_distance(screen_size_x, screen_size_y);
            let view_x = 2.0 / (screen_size_x * scale);
            let view_y = -2.0 / (screen_size_y * scale);
            let offset = screen_size_y * scale * OFFSET_RATIO;
            let mut is_camera = false;

            #[allow(clippy::indexing_slicing)]
            for (camera, transform) in (&mut cameras, &mut transforms).join() {
                // Keep in sync with `Camera::standard_2d` source
                camera.matrix[(0, 0)] = view_x;
                camera.matrix[(1, 1)] = view_y;
                camera.matrix[(0, 3)] = 0.0;
                camera.matrix[(1, 3)] = 0.0;
                transform.set_translation_y(offset);
                is_camera = true;
            }

            if is_camera {
                self.screen_size_x = screen_size_x;
                self.screen_size_y = screen_size_y;
            }
        }
    }
}

fn to_view_distance(size_x: f32, size_y: f32) -> f32 {
    return (size_x * size_x + size_y * size_y).sqrt();
}
