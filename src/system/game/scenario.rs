use crate::resource::Scenario;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::prelude::Commands;
use bevy::time::Time;

pub fn scenario(mut scenario: ResMut<Scenario>, mut commands: Commands, time: Res<Time>) {
    scenario.update(&mut commands, time.elapsed());
}
