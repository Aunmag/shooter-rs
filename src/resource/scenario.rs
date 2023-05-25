use bevy::{ecs::system::Resource, prelude::Commands};
use std::{any::Any, time::Duration};

pub trait ScenarioLogic {
    fn update(&mut self, commands: &mut Commands) -> Duration;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Resource)]
pub struct Scenario {
    logic: Box<dyn ScenarioLogic + 'static + Send + Sync>,
    timer: Duration,
}

impl Scenario {
    pub fn new<T: ScenarioLogic + 'static + Send + Sync>(logic: T) -> Self {
        return Self {
            logic: Box::new(logic),
            timer: Duration::ZERO,
        };
    }

    pub fn update(&mut self, commands: &mut Commands, time: Duration) {
        while self.timer < time {
            self.timer = time + self.logic.update(commands);
        }
    }

    pub fn logic<T: ScenarioLogic + 'static>(&mut self) -> Option<&mut T> {
        return self.logic.as_mut().as_any_mut().downcast_mut::<T>();
    }
}
