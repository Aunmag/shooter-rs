use crate::component::{Bot, Voice, VoiceSound};
use bevy::{ecs::system::Res, prelude::Query, time::Time};
use rand::{thread_rng, Rng as _};
use std::{ops::Range, time::Duration};

const INTERVAL: Range<Duration> = Duration::from_secs(2)..Duration::from_secs(30);

// TODO: rework
pub fn sound(mut bots: Query<(&mut Bot, &mut Voice)>, time: Res<Time>) {
    let time = time.elapsed();

    for (mut bot, mut voice) in bots.iter_mut() {
        if time < bot.next_sound {
            continue;
        }

        if !bot.next_sound.is_zero() {
            voice.queue(VoiceSound::Misc, time);
        }

        bot.next_sound = time + thread_rng().gen_range(INTERVAL);
    }
}
