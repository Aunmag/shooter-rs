use crate::event::ActorDeathEvent;
use bevy::{
    ecs::{event::EventReader, system::Resource},
    prelude::Commands,
};
use std::{any::Any, time::Duration};

pub trait ScenarioLogic {
    fn on_start(&mut self, _commands: &mut Commands) -> Duration {
        return Duration::ZERO;
    }

    fn on_actor_death(&mut self, _event: &ActorDeathEvent, _commands: &mut Commands) {}

    fn on_player_death(&mut self, _event: &ActorDeathEvent, _commands: &mut Commands) {}

    fn on_interval_update(&mut self, _commands: &mut Commands) -> Duration {
        return Duration::from_secs(60);
    }

    fn on_constant_update(&mut self, _commands: &mut Commands) {}

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Resource)]
pub struct Scenario {
    logic: Box<dyn ScenarioLogic + 'static + Send + Sync>,
    timer: Duration,
    is_started: bool,
}

impl Scenario {
    pub fn new<T: ScenarioLogic + 'static + Send + Sync>(logic: T) -> Self {
        return Self {
            logic: Box::new(logic),
            timer: Duration::ZERO,
            is_started: false,
        };
    }

    pub fn update(
        &mut self,
        commands: &mut Commands,
        mut death_events: EventReader<ActorDeathEvent>,
        time: Duration,
    ) {
        if !self.is_started {
            self.timer = time + self.logic.on_start(commands);
            self.is_started = true;
        }

        if !death_events.is_empty() {
            for event in death_events.read() {
                self.logic.on_actor_death(event, commands);

                if event.is_player {
                    self.logic.on_player_death(event, commands);
                }
            }
        }

        if self.timer <= time {
            self.timer = time + self.logic.on_interval_update(commands);
        }

        self.logic.on_constant_update(commands);
    }

    pub fn logic<T: ScenarioLogic + 'static>(&mut self) -> Option<&mut T> {
        return self.logic.as_mut().as_any_mut().downcast_mut::<T>();
    }
}
