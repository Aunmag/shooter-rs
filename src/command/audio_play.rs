use crate::component::Player;
use crate::resource::AudioStorage;
use crate::resource::Rng;
use bevy::ecs::query::With;
use bevy::ecs::system::Command;
use bevy::math::Vec2;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Audio;
use bevy::prelude::PlaybackSettings;
use bevy::prelude::World;
use bevy::transform::components::Transform;
use rand::Rng as _;

const VOLUME_MIN: f32 = 0.01;

#[derive(Clone)]
pub struct AudioPlay {
    pub path: &'static str,
    pub volume: f32,
    pub repeat: bool,
    pub chance: f32,
    pub source: Option<Vec2>,
}

impl AudioPlay {
    pub const DEFAULT: Self = Self {
        path: "sounds/default.ogg",
        volume: 1.0,
        repeat: false,
        chance: 1.0,
        source: None,
    };

    pub fn as_spatial(&self, source: Vec2) -> Self {
        return Self {
            source: Some(source),
            ..self.clone()
        };
    }

    pub fn should_play(&self) -> bool {
        return self.volume > VOLUME_MIN && self.chance > 0.0;
    }

    pub fn settings(&self) -> PlaybackSettings {
        let settings = if self.repeat {
            PlaybackSettings::LOOP
        } else {
            PlaybackSettings::ONCE
        };

        return settings.with_volume(self.volume);
    }
}

impl Command for AudioPlay {
    fn write(mut self, world: &mut World) {
        if !self.should_play() {
            return;
        }

        if self.chance < 1.0 && !world.resource_mut::<Rng>().gen_bool(self.chance.into()) {
            return;
        }

        if let Some(source) = self.source {
            if let Some(listener) = world
                .query_filtered::<&Transform, With<Player>>()
                .iter(world)
                .next()
            {
                self.volume = calc_spatial_volume(listener.translation.xy(), source, self.volume);
            }
        }

        let handle = if let Some(handle) = world.resource_mut::<AudioStorage>().choose(self.path) {
            handle
        } else {
            log::warn!("Audio {} not found", self.path);
            return;
        };

        world
            .resource::<Audio>()
            .play_with_settings(handle, self.settings());
    }
}

fn calc_spatial_volume(listener: Vec2, source: Vec2, volume: f32) -> f32 {
    return f32::min(1.0 / source.distance(listener).sqrt(), 1.0) * volume;
}
