use crate::{
    command::CursorGrab, model::AudioPlay, resource::AudioTracker,
};
use bevy::{
    ecs::{system::Command, world::World},
    prelude::Camera2dBundle,
};
use crate::tool::world_generator::WorldGenerator;

pub fn on_enter(world: &mut World) {
    CursorGrab(true).apply(world);
    world.spawn(Camera2dBundle::default());
    play_audio(world);
    WorldGenerator::new(world, 200).generate(); // TODO: get seed from config
}

fn play_audio(world: &mut World) {
    let audio = world.resource::<AudioTracker>();

    audio.queue(AudioPlay {
        path: "sounds/ambience_music".into(),
        volume: 0.3,
        duration: AudioPlay::DURATION_FOREVER,
        ..AudioPlay::DEFAULT
    });

    audio.queue(AudioPlay {
        path: "sounds/ambience_nature".into(),
        volume: 0.2,
        duration: AudioPlay::DURATION_FOREVER,
        ..AudioPlay::DEFAULT
    });
}
