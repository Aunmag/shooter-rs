use crate::{event::ActorDeathEvent, resource::Scenario};
use bevy::{
    ecs::system::{Res, ResMut},
    prelude::{Commands, EventReader},
    time::Time,
};

pub fn scenario(
    mut scenario: ResMut<Scenario>,
    mut commands: Commands,
    death_events: EventReader<ActorDeathEvent>,
    time: Res<Time>,
) {
    scenario.update(&mut commands, death_events, time.elapsed());
}
