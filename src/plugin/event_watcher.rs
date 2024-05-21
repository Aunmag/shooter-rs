use crate::{model::AppState, util::ext::AppExt};
use bevy::{
    app::Update, ecs::event::Event, prelude::{App, Commands, IntoSystemConfigs, Plugin, Res, Time, World}
};

type Command = fn(&mut World);

pub struct EventWatcherPlugin;

impl Plugin for EventWatcherPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameEvent>();
        // app.add_state_system(AppState::Game, on_update);
        // app.add_systems(Update, on_update);
    }
}

#[derive(Event)]
pub enum GameEvent {
    ActorSpawn(Entity),
    ActorDeath(Entity),
    PlayerSpawn(Entity),
    PlayerDeath(Entity),
}

// TODO: generate
pub enum GameEventDiscriminant {}

pub struct EventWatcher {
    subscribers: Vec<(GameEventDiscriminant, Command)>,
}

impl EventWatcher {
    pub fn subscribe(&mut self, event: GameEventDiscriminant, command: Command) {
        self.subscribers.push(event, command);
    }
}

// pub fn scenario(
//     mut scenario: ResMut<Scenario>,
//     mut commands: Commands,
//     death_events: EventReader<ActorDeathEvent>,
//     time: Res<Time>,
// ) {
//     scenario.update(&mut commands, death_events, time.elapsed());
// }

fn on_update(
    mut commands: Commands,
    time: Res<Time>,
) {
    let time = time.elapsed();
    todo!();
}
