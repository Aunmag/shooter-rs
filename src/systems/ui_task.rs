use crate::resources::UiTask;
use crate::resources::UiTaskResource;
use amethyst::ecs::Join;
use amethyst::ecs::System;
use amethyst::ecs::Write;
use amethyst::ecs::WriteStorage;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;
use std::collections::HashMap;

pub struct UiTaskSystem;

impl UiTaskSystem {
    fn prepare_tasks(tasks: &mut UiTaskResource) {
        if !tasks.is_empty() {
            let mut tasks_additional = HashMap::new();

            for (id, task) in tasks.iter() {
                if let UiTask::SetButtonAvailability(is_availability) = *task {
                    tasks_additional.insert(
                        format!("{}_btn_txt", id),
                        UiTask::SetButtonAvailability(is_availability),
                    );
                }
            }

            for (id, task) in tasks_additional.drain() {
                tasks.insert(id, task);
            }
        }
    }
}

impl<'a> System<'a> for UiTaskSystem {
    type SystemData = (
        Write<'a, UiTaskResource>,
        WriteStorage<'a, UiText>,
        WriteStorage<'a, UiTransform>,
    );

    fn run(&mut self, (mut tasks, mut texts, mut transforms): Self::SystemData) {
        if tasks.is_empty() {
            return;
        }

        Self::prepare_tasks(&mut tasks);

        for (transform, text) in (&mut transforms, (&mut texts).maybe()).join() {
            if let Some(task) = tasks.remove(&transform.id) {
                match task {
                    UiTask::SetButtonAvailability(is_availability) => {
                        if let Some(text) = text {
                            if is_availability {
                                text.color[3] = 1.0;
                            } else {
                                text.color[3] = 0.3;
                            }
                        } else {
                            transform.opaque = is_availability;
                        }
                    }
                    UiTask::SetText(text_to_set) => {
                        if let Some(text) = text {
                            text.text = text_to_set.to_string();
                        }
                    }
                }

                if tasks.is_empty() {
                    break;
                }
            }
        }

        tasks.clear();
    }
}
