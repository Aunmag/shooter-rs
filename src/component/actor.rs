use crate::command::AudioPlay;
use crate::model::ActorActions;
use crate::model::SpriteOffset;
use bevy::ecs::component::Component;
use serde::Deserialize;
use serde::Serialize;
use std::f32::consts::TAU;
use std::time::Duration;

#[derive(Component)]
pub struct Actor {
    pub config: &'static ActorConfig,
    pub actions: ActorActions,
    pub look_at: Option<f32>,
    pub melee_next: Duration,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActorType {
    Human,
    Zombie,
}

pub struct ActorConfig {
    pub sprite: &'static str,
    pub sprite_offset: SpriteOffset,
    pub movement_velocity: f32,
    pub rotation_velocity: f32,
    pub sprint_factor: f32,
    pub resistance: f32,
    pub radius: f32,
    pub mass: f32,
    pub melee_damage: f32,
    pub melee_distance: f32,
    pub melee_distance_angular: f32,
    pub melee_interval: Duration,
    pub actor_type: ActorType,
    pub pain_threshold: f32,
    pub sound_pain: Option<AudioPlay>,
    pub sound_death: Option<AudioPlay>,
}

impl Actor {
    pub const fn new(config: &'static ActorConfig) -> Self {
        return Self {
            config,
            actions: ActorActions::EMPTY,
            look_at: None,
            melee_next: Duration::ZERO,
        };
    }
}

impl ActorConfig {
    const HUMAN_RESISTANCE: f32 = 8000.0;

    pub const HUMAN: &'static Self = &Self {
        sprite: "actors/human/image.png",
        sprite_offset: SpriteOffset::new(Some(9.0), None),
        movement_velocity: 2.5,
        rotation_velocity: 8.0,
        sprint_factor: 2.0,
        resistance: Self::HUMAN_RESISTANCE,
        radius: 0.25,
        mass: 80_000.0,
        melee_damage: Self::HUMAN_RESISTANCE / 16.0, // 16 hits to kill human
        melee_distance: 0.7,
        melee_distance_angular: TAU / 3.0,
        melee_interval: Duration::from_millis(400),
        actor_type: ActorType::Human,
        pain_threshold: 0.02,
        sound_pain: Some(AudioPlay {
            path: "sounds/human_pain_{n}.ogg",
            volume: 0.8,
            chance: 1.0,
            ..AudioPlay::DEFAULT
        }),
        sound_death: None,
    };

    pub const ZOMBIE: &'static Self = &Self {
        sprite: "actors/zombie/image.png",
        sprite_offset: SpriteOffset::new(Some(6.5), None),
        movement_velocity: Self::HUMAN.movement_velocity * 0.4,
        rotation_velocity: Self::HUMAN.rotation_velocity * 0.4,
        sprint_factor: 1.8,
        resistance: Self::HUMAN.resistance * 0.4,
        radius: 0.21,
        mass: 70_000.0,
        melee_damage: Self::HUMAN.resistance / 8.0, // 8 hits to kill human
        melee_distance: Self::HUMAN.melee_distance,
        melee_distance_angular: Self::HUMAN.melee_distance_angular,
        melee_interval: Self::HUMAN.melee_interval,
        actor_type: ActorType::Zombie,
        pain_threshold: 0.08,
        sound_pain: Some(AudioPlay {
            path: "sounds/zombie_pain_{n}.ogg",
            volume: 1.0,
            chance: 0.3,
            ..AudioPlay::DEFAULT
        }),
        sound_death: Some(AudioPlay {
            path: "sounds/zombie_death_{n}.ogg",
            volume: 1.0,
            chance: 0.4,
            ..AudioPlay::DEFAULT
        }),
    };
}

impl From<ActorType> for &'static ActorConfig {
    fn from(actor_type: ActorType) -> Self {
        return match actor_type {
            ActorType::Human => ActorConfig::HUMAN,
            ActorType::Zombie => ActorConfig::ZOMBIE,
        };
    }
}
