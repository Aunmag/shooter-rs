use crate::{model::AppState, util::ext::AppExt};
use bevy::{
    ecs::system::{ResMut, Resource},
    prelude::{App, Commands, Plugin, Res, Time},
};
use std::time::Duration;

pub struct CommandSchedulerPlugin;

impl Plugin for CommandSchedulerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update);
    }
}

// TODO: just accept static fn?
type Foo = Box<dyn ScheduledCommand + 'static + Send + Sync>; // TODO: rename

#[derive(Resource)]
pub struct CommandScheduler {
    tasks: Vec<Task>,
    next: Duration,
}

impl CommandScheduler {
    pub fn add(&mut self, task: Foo, start: Duration, interval: Duration) {
        self.tasks.push(Task {
            imp: task,
            interval,
            next: start,
        });

        if self.next > start {
            self.next = start;
        }
    }
}

struct Task {
    imp: Foo, // TODO: rename
    interval: Duration,
    next: Duration,
}

fn on_update(mut scheduler: ResMut<CommandScheduler>, mut commands: Commands, time: Res<Time>) {
    let time = time.elapsed();

    if time > scheduler.next {
        return;
    }

    let mut new_next = Duration::MAX;

    for task in &mut scheduler.tasks {
        if task.next <= time {
            task.imp.apply(&mut commands);
            task.next = time + task.interval;
        }

        if task.next < new_next {
            new_next = task.next;
        }
    }

    scheduler.next = new_next;
}

pub trait ScheduledCommand {
    // TODO: pass time?
    fn apply(&self, commands: &mut Commands); // TODO: simplify
}
