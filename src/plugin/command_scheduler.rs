use crate::{model::AppState, util::ext::AppExt};
use bevy::prelude::{App, Commands, Plugin, Res, ResMut, Resource, Time, World};
use std::{any::TypeId, time::Duration};
use std::any::Any;

pub struct CommandSchedulerPlugin;

impl Plugin for CommandSchedulerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update);
    }
}

type Command = fn(&mut World);

#[derive(Resource)]
pub struct CommandScheduler {
    commands: Vec<ScheduledCommand>,
    next_run: Duration,
}

impl CommandScheduler {
    pub fn add(&mut self, interval: Duration, command: Command) {
        let next_run = Duration::ZERO;

        self.commands.push(ScheduledCommand {
            command,
            interval,
            next_run,
        });

        if self.next_run > next_run {
            self.next_run = next_run;
        }
    }

    pub fn reschedule(&mut self, id: TypeId, interval: Duration) {
        for command in &mut self.commands {
            if command.command.type_id() == id {
                if !command.next_run.is_zero() {
                    let last_run = command.next_run.saturating_sub(command.interval);
                    command.next_run = last_run + interval;
                }

                command.interval = interval;
                break;
            }
        }

        debug_assert!(false, "Scheduled command not found");
    }
}

struct ScheduledCommand {
    command: Command,
    interval: Duration,
    next_run: Duration,
}

fn on_update(mut scheduler: ResMut<CommandScheduler>, mut commands: Commands, time: Res<Time>) {
    let now = time.elapsed();

    if now < scheduler.next_run {
        return;
    }

    let mut next_run = Duration::MAX;

    for command in &mut scheduler.commands {
        if command.next_run <= now {
            commands.add(command.command);
            command.next_run = now + command.interval;
        }

        if next_run > command.next_run {
            next_run = command.next_run;
        }
    }

    scheduler.next_run = next_run;
}
