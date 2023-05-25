use crate::command::AudioPlay;
use crate::resource::Rng;
use bevy::prelude::Commands;
use bevy::prelude::Vec2;
use rand::Rng as _;

pub struct SoundConfig {
    pub path: &'static str,
    pub choices: u8, // TODO: detect automatically
    pub volume: f32,
    pub chance: f32,
}

impl SoundConfig {
    pub const fn new(path: &'static str, choices: u8, volume: f32, chance: f32) -> Self {
        return Self {
            path,
            choices,
            volume,
            chance,
        };
    }

    pub fn play(&self, source: Vec2, commands: &mut Commands) {
        commands.add(AudioPlay {
            path: self.path,
            volume: self.volume,
            repeat: false,
            source: Some(source),
            choices: self.choices,
        });
    }

    pub fn maybe_play(&self, source: Vec2, rng: &mut Rng, commands: &mut Commands) {
        if rng.gen_bool(self.chance.into()) {
            self.play(source, commands);
        }
    }
}
