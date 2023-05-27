use crate::{command::AudioPlay, component::Bot};
use bevy::{
    ecs::system::Res,
    math::Vec3Swizzles,
    prelude::{Commands, Query, Transform},
    time::Time,
};
use rand::{thread_rng, Rng as _};
use std::time::Duration;

const INTERVAL_MIN: f32 = 2.0;
const INTERVAL_MAX: f32 = 30.0;

pub fn sound(mut bots: Query<(&mut Bot, &Transform)>, mut commands: Commands, time: Res<Time>) {
    let time = time.elapsed();

    for (mut bot, transform) in bots.iter_mut() {
        if time < bot.next_sound {
            continue;
        }

        if !bot.next_sound.is_zero() {
            commands.add(AudioPlay {
                path: "sounds/zombie_{n}.ogg",
                volume: 0.7,
                source: Some(transform.translation.xy()),
                ..AudioPlay::DEFAULT
            });
        }

        let interval = Duration::from_secs_f32(thread_rng().gen_range(INTERVAL_MIN..INTERVAL_MAX));
        bot.next_sound = time + interval;
    }
}
