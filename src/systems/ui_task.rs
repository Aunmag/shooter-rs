use crate::resources::UiTask;
use crate::resources::UiTaskResource;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::Write;
use amethyst::ecs::prelude::WriteStorage;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;
use std::collections::HashMap;

const FONT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const FONT_COLOR_DISABLED: [f32; 4] = [0.8, 0.8, 0.8, 0.5];

#[derive(SystemDesc)]
pub struct UiTaskSystem;

impl<'a> System<'a> for UiTaskSystem {
    type SystemData = (
        Write<'a, UiTaskResource>,
        WriteStorage<'a, UiText>,
        WriteStorage<'a, UiTransform>,
    );

    fn run(&mut self, (mut tasks, mut texts, mut transforms): Self::SystemData) {
        let mut to_update = HashMap::new();

        while let Some(task) = tasks.pop() {
            match task {
                UiTask::SetButtonAvailability(id, is_availability) => {
                    to_update.insert(id.to_string(), is_availability);
                    to_update.insert(format!("{}_btn_txt", id), is_availability);
                }
            }
        }

        for (transform, text) in (&mut transforms, (&mut texts).maybe()).join() {
            if let Some(is_availability) = to_update.remove(&transform.id) {
                if let Some(text) = text {
                    if is_availability {
                        text.color = FONT_COLOR;
                    } else {
                        text.color = FONT_COLOR_DISABLED;
                    }
                } else {
                    transform.opaque = is_availability;
                }
            }

            if to_update.is_empty() {
                break;
            }
        }
    }
}
