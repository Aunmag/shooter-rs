use crate::{
    command::CursorGrab,
    data::{LAYER_GROUND, LAYER_TREE, WORLD_SIZE, WORLD_SIZE_HALF, WORLD_SIZE_VISUAL},
    model::AudioPlay,
    plugin::{TileBlend, WorldGenerator},
    resource::AudioTracker,
    util::ext::Vec2Ext,
};
use bevy::{
    asset::AssetServer,
    ecs::{system::Command, world::World},
    math::{Vec2, Vec3},
    prelude::Camera2dBundle,
};
use rand::{seq::SliceRandom, Rng};
use std::f32::consts::{FRAC_PI_2, PI, TAU};

pub fn on_enter(world: &mut World) {
    CursorGrab(true).apply(world);
    world.spawn(Camera2dBundle::default());
    play_audio(world);
    WorldGenerator::new(world, 200).generate();
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
