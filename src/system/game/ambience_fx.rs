use crate::command::AudioPlay;
use crate::resource::Rng;
use bevy::ecs::system::Res;
use bevy::ecs::system::Resource;
use bevy::prelude::Commands;
use bevy::prelude::ResMut;
use bevy::time::Time;
use rand::Rng as _;
use std::time::Duration;

const INTERVAL_MIN: f32 = 15.0;
const INTERVAL_MAX: f32 = 30.0;

#[derive(Default, Resource)]
pub struct AmbienceFxData {
    next: Duration,
}

pub fn ambience_fx(
    mut data: ResMut<AmbienceFxData>,
    mut rng: ResMut<Rng>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let time = time.elapsed();

    if time < data.next {
        return;
    }

    if !data.next.is_zero() {
        commands.add(AudioPlay {
            path: "sounds/ambience_fx_{n}.ogg",
            volume: 0.2,
            choices: 7,
            ..Default::default()
        });
    }

    let interval = Duration::from_secs_f32(rng.gen_range(INTERVAL_MIN..INTERVAL_MAX));
    data.next = time + interval;
}
