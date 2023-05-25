use crate::resource::Scenario;
use bevy::{
    ecs::system::{Res, ResMut},
    prelude::Commands,
    time::Time,
};

pub fn scenario(mut scenario: ResMut<Scenario>, mut commands: Commands, time: Res<Time>) {
    scenario.update(&mut commands, time.elapsed());
}
