use crate::utils::UiAwaiter;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::ReadExpect;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::WriteStorage;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;
use amethyst::window::ScreenDimensions;

/// Determines which screen min(width, height) font sizes are set to by default
const ORIGIN_SCREEN_SIZE_QUAD: f32 = 720.0;
const WALLPAPER_SIZE_X: f32 = 480.0;
const WALLPAPER_SIZE_Y: f32 = 270.0;
const WALLPAPER_ASPECT_RATIO: f32 = WALLPAPER_SIZE_X / WALLPAPER_SIZE_Y;

#[derive(SystemDesc)]
pub struct UiResizeSystem {
    ui_awaiter: UiAwaiter,
    last_size_x: f32,
    last_size_y: f32,
    last_size_quad: f32,
}

impl UiResizeSystem {
    pub fn new() -> Self {
        return Self {
            ui_awaiter: UiAwaiter::new(),
            last_size_x: 0.0,
            last_size_y: 0.0,
            last_size_quad: 0.0,
        };
    }

    fn resize_wallpapers(transforms: &mut WriteStorage<UiTransform>, size_x: f32, size_y: f32) {
        let scale;

        if size_x / size_y > WALLPAPER_ASPECT_RATIO {
            scale = size_x / WALLPAPER_SIZE_X;
        } else {
            scale = size_y / WALLPAPER_SIZE_Y;
        }

        for transform in (transforms).join() {
            if transform.id == "confirm.wallpaper"
                || transform.id == "home.wallpaper"
                || transform.id == "new_game.wallpaper"
                || transform.id == "quit.wallpaper"
            {
                transform.width = WALLPAPER_SIZE_X * scale;
                transform.height = WALLPAPER_SIZE_Y * scale;
            }
        }
    }

    fn resize_texts(&self, texts: &mut WriteStorage<UiText>, size_quad: f32) {
        let last_size_quad;

        if self.last_size_quad == 0.0 {
            last_size_quad = ORIGIN_SCREEN_SIZE_QUAD;
        } else {
            last_size_quad = self.last_size_quad;
        }

        let scale = size_quad / last_size_quad;

        for text in (texts).join() {
            text.font_size *= scale;
        }
    }
}

impl<'a> System<'a> for UiResizeSystem {
    type SystemData = (
        ReadExpect<'a, ScreenDimensions>,
        WriteStorage<'a, UiText>,
        WriteStorage<'a, UiTransform>,
    );

    fn run(&mut self, (screen, mut texts, mut transforms): Self::SystemData) {
        self.ui_awaiter.update();

        if self.ui_awaiter.is_ready() {
            let size_x = screen.width();
            let size_y = screen.height();
            let size_quad = f32::min(size_y, size_x);

            #[allow(clippy::float_cmp)]
            if size_x != self.last_size_x || size_y != self.last_size_y {
                Self::resize_wallpapers(&mut transforms, size_x, size_y);
                self.last_size_x = size_x;
                self.last_size_y = size_y;
            }

            #[allow(clippy::float_cmp)]
            if size_quad != self.last_size_quad {
                self.resize_texts(&mut texts, size_quad);
                self.last_size_quad = size_quad;
            }
        }
    }
}
