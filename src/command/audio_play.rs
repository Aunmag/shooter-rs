use crate::component::Player;
use crate::resource::Rng;
use bevy::ecs::system::Command;
use bevy::math::Vec3Swizzles;
use bevy::prelude::AssetServer;
use bevy::prelude::Audio;
use bevy::prelude::PlaybackSettings;
use bevy::prelude::Transform;
use bevy::prelude::Vec2;
use bevy::prelude::With;
use bevy::prelude::World;
use rand::Rng as _;

const VOLUME_MIN: f32 = 0.01;

pub struct AudioPlay {
    pub path: &'static str,
    pub volume: f32,
    pub repeat: bool,
    pub source: Option<Vec2>,
    pub choices: u8, // TODO: detect automatically
}

impl Default for AudioPlay {
    fn default() -> Self {
        return Self {
            path: "sound/shot.ogg",
            source: None,
            volume: 1.0,
            repeat: false,
            choices: 1,
        };
    }
}

impl AudioPlay {
    fn get_random_path(&self, rng: &mut Rng) -> String {
        let n = rng.gen_range(0..self.choices);
        let n_string = if self.choices > 9 {
            format!("{:02}", n)
        } else {
            format!("{}", n)
        };

        return self.path.replace("{n}", &n_string);
    }
}

impl Command for AudioPlay {
    fn write(mut self, world: &mut World) {
        if let Some(source) = self.source {
            if let Some(listener) = world
                .query_filtered::<&Transform, With<Player>>()
                .iter(world)
                .next()
            {
                self.volume = calc_volume(listener.translation.xy(), source, self.volume);
            } else {
                return;
            }
        }

        if self.volume < VOLUME_MIN {
            return;
        }

        let handle = if self.choices > 1 {
            let path = self.get_random_path(&mut world.resource_mut::<Rng>());
            world.resource::<AssetServer>().get_handle(path)
        } else {
            world.resource::<AssetServer>().get_handle(self.path)
        };

        let settings = if self.repeat {
            PlaybackSettings::LOOP
        } else {
            PlaybackSettings::ONCE
        };

        world
            .resource::<Audio>()
            .play_with_settings(handle, settings.with_volume(self.volume));
    }
}

fn calc_volume(listener: Vec2, source: Vec2, volume: f32) -> f32 {
    return f32::min(1.0 / source.distance(listener).sqrt(), 1.0) * volume;
}
