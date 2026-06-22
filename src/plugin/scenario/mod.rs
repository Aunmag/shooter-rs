mod bench;
mod test;
mod test_bot_spread;
mod waves;

pub use self::{bench::*, test::*, test_bot_spread::*, waves::*};
use crate::{event::ActorDeathEvent, model::AppState, util::ext::AppExt};
use bevy::{
    ecs::{
        event::EventReader,
        system::{Res, ResMut, Resource},
        world::{Mut, World},
    },
    prelude::{App, Commands, Plugin},
    time::Time,
};
use std::{any::Any, time::Duration};

pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system_enter(AppState::Game, on_enter);
        app.add_state_system(AppState::Game, on_update);
    }
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

    fn logic<T: ScenarioLogic + 'static>(&mut self) -> Option<&mut T> {
        return self.logic.as_mut().as_any_mut().downcast_mut::<T>();
    }
}

pub trait ScenarioLogic {
    fn on_enter(&mut self, _world: &mut World) -> Duration {
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

fn on_enter(world: &mut World) {
    world.resource_scope(|world, mut scenario: Mut<Scenario>| {
        let time = world.resource::<Time>().elapsed();
        let timeout = scenario.logic.on_enter(world);
        scenario.timer = time + timeout;
    });
}

// TODO: skip if no scenario resource?
fn on_update(
    mut scenario: ResMut<Scenario>,
    mut commands: Commands,
    mut death_events: EventReader<ActorDeathEvent>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    if !death_events.is_empty() {
        for event in death_events.read() {
            scenario.logic.on_actor_death(event, &mut commands);

            if event.is_player {
                scenario.logic.on_player_death(event, &mut commands);
            }
        }
    }

    if scenario.timer <= time {
        scenario.timer = time + scenario.logic.on_interval_update(&mut commands);
    }

    scenario.logic.on_constant_update(&mut commands);
}
