use crate::components::Health;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use amethyst::core::timing::Time;
use amethyst::ecs::Entities;
use amethyst::ecs::Join;
use amethyst::ecs::Read;
use amethyst::ecs::ReadStorage;
use amethyst::ecs::System;
use amethyst::ecs::Write;

pub struct HealthSystem;

impl<'a> System<'a> for HealthSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, Health>,
        Write<'a, GameTaskResource>,
    );

    fn run(&mut self, (entities, time, healths, mut tasks): Self::SystemData) {
        let now = time.absolute_time();

        for (entity, health) in (&entities, &healths).join() {
            if health.is_decayed(now) {
                tasks.push(GameTask::EntityDelete(entity));
            }
        }
    }
}
