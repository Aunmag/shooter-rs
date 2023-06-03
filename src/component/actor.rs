use crate::{
    model::{ActorActions, ActorActionsExt, AudioPlay},
    util::ext::DurationExt,
};
use bevy::ecs::component::Component;
use std::{f32::consts::TAU, time::Duration};

#[derive(Component)]
pub struct Actor {
    pub config: &'static ActorConfig,
    pub skill: f32, // TODO: affect bots reaction too
    pub stamina: f32,
    pub actions: ActorActions,
    pub look_at: Option<f32>,
    pub melee_next: Duration,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActorType {
    Human,
    Zombie,
}

pub struct ActorConfig {
    pub movement_velocity: f32,
    pub rotation_velocity: f32,
    pub sprint_factor: f32,
    pub stamina: Duration,
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
    pub const ARMS_LENGTH_1: f32 = 0.546875;
    pub const ARMS_LENGTH_2: f32 = 0.34375;

    pub const fn new(config: &'static ActorConfig, skill: f32) -> Self {
        return Self {
            config,
            skill,
            stamina: 1.0,
            actions: ActorActions::EMPTY,
            look_at: None,
            melee_next: Duration::ZERO,
        };
    }

    pub fn update_stamina(&mut self, delta: f32) {
        let mut change = self.config.stamina.rate() * delta;

        if self.actions.is_moving() {
            if self.actions.is_sprinting() {
                // spend stamina while sprinting
                change = -change;
            } else {
                // slower stamina gain while just moving
                change *= self.stamina / 2.0;
            }
        }

        self.stamina = (self.stamina + change).clamp(0.0, 1.0);
    }
}

impl ActorConfig {
    const HUMAN_RESISTANCE: f32 = 8000.0;

    pub const HUMAN: Self = Self {
        movement_velocity: 2.5,
        rotation_velocity: 8.0,
        sprint_factor: 2.0,
        stamina: Duration::from_secs(16),
        resistance: Self::HUMAN_RESISTANCE,
        radius: 0.25,
        mass: 80_000.0,
        melee_damage: Self::HUMAN_RESISTANCE / 16.0, // 16 hits to kill human
        melee_distance: 0.7,
        melee_distance_angular: TAU / 5.0,
        melee_interval: Duration::from_millis(400),
        actor_type: ActorType::Human,
        pain_threshold: 0.02,
        sound_pain: Some(AudioPlay {
            path: "sounds/human_pain_{n}.ogg",
            volume: 0.8,
            chance: 1.0,
            priority: AudioPlay::PRIORITY_HIGHER,
            ..AudioPlay::DEFAULT
        }),
        sound_death: None,
    };

    pub const ZOMBIE: Self = Self {
        movement_velocity: Self::HUMAN.movement_velocity * 0.4,
        rotation_velocity: Self::HUMAN.rotation_velocity * 0.4,
        sprint_factor: 1.8,
        stamina: Duration::from_secs(10),
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
            priority: AudioPlay::PRIORITY_LOWER,
            ..AudioPlay::DEFAULT
        }),
        sound_death: Some(AudioPlay {
            path: "sounds/zombie_death_{n}.ogg",
            volume: 1.0,
            chance: 0.4,
            priority: AudioPlay::PRIORITY_MEDIUM,
            ..AudioPlay::DEFAULT
        }),
    };

    pub fn get_image_path(&self, mut suffix: u8) -> String {
        let folder;

        match self.actor_type {
            ActorType::Human => {
                folder = "human";
                suffix = suffix.clamp(1, 2); // since player has only 1 and 2
            }
            ActorType::Zombie => {
                folder = "zombie";
                suffix = 0; // since zombie only has 0
            }
        };

        return format!("actors/{}/image_{}.png", folder, suffix);
    }
}

impl From<ActorType> for &'static ActorConfig {
    fn from(actor_type: ActorType) -> Self {
        return match actor_type {
            ActorType::Human => &ActorConfig::HUMAN,
            ActorType::Zombie => &ActorConfig::ZOMBIE,
        };
    }
}

#[derive(Component)]
pub struct ActorWeaponSprite;
