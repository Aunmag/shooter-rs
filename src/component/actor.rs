use crate::{
    data::BotConfig,
    model::{ActorActions, ActorActionsExt},
    util::ext::{DurationExt, Vec2Ext},
};
use bevy::{ecs::component::Component, math::Vec2};
use std::{f32::consts::TAU, time::Duration};

#[derive(Component)]
pub struct Actor {
    pub config: &'static ActorConfig,
    pub skill: f32,
    pub stamina: f32,
    pub movement: Vec2,
    pub actions: ActorActions,
    pub look_at: Option<f32>,
    pub melee_next: Duration,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActorKind {
    Human,
    Zombie,
}

pub struct ActorConfig {
    pub kind: ActorKind,
    pub name: &'static str,
    // movement
    pub movement_velocity: f32,
    pub rotation_velocity: f32,
    pub sprint_factor: f32,
    pub stamina: Duration,
    // health
    pub health: f32,
    pub pain_threshold: f32,
    // physics
    pub radius: f32,
    pub mass: f32,
    // melee
    pub melee_damage: f32,
    pub melee_distance: f32,
    pub melee_distance_angular: f32,
    pub melee_interval: Duration,
    // shooting
    pub reloading_speed: f32,
    pub recoil_factor: f32,
    // misc
    pub bot: &'static BotConfig,
    pub images: &'static [u8],
}

impl Actor {
    pub const ARMS_LENGTH_1: f32 = 0.546875;
    pub const ARMS_LENGTH_2: f32 = 0.34375;

    pub const fn new(config: &'static ActorConfig, skill: f32) -> Self {
        return Self {
            config,
            skill,
            stamina: 1.0,
            movement: Vec2::ZERO,
            actions: ActorActions::EMPTY,
            look_at: None,
            melee_next: Duration::ZERO,
        };
    }

    pub fn reset_actions(&mut self) {
        self.movement = Vec2::ZERO;
        self.actions = ActorActions::EMPTY;
        self.look_at = None;
    }

    pub fn update_stamina(&mut self, delta: f32) {
        let mut change = self.config.stamina.mul_f32(self.skill).delta(delta);

        if !self.movement.is_zero() {
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
    const HUMAN_HEALTH: f32 = 9.0;

    pub const HUMAN: Self = Self {
        kind: ActorKind::Human,
        name: "human",
        movement_velocity: 2.8,
        rotation_velocity: 3.5,
        sprint_factor: 1.6,
        stamina: Duration::from_secs(16),
        health: Self::HUMAN_HEALTH,
        pain_threshold: 0.02,
        radius: 0.25,
        mass: 85.0,
        melee_damage: Self::HUMAN_HEALTH / 16.0,
        melee_distance: 0.7,
        melee_distance_angular: TAU / 5.0,
        melee_interval: Duration::from_millis(600),
        reloading_speed: 0.6,
        recoil_factor: 1.0,
        bot: BotConfig::HUMAN,
        images: &[1, 2],
    };

    pub const ZOMBIE: Self = Self {
        kind: ActorKind::Zombie,
        name: "zombie",
        movement_velocity: Self::HUMAN.movement_velocity * 0.33,
        rotation_velocity: Self::HUMAN.rotation_velocity * 0.4,
        sprint_factor: Self::HUMAN.sprint_factor,
        stamina: Duration::from_secs(10),
        health: Self::HUMAN.health / 2.0,
        pain_threshold: f32::INFINITY, // disabled
        radius: 0.21,
        mass: 70.0,
        melee_damage: Self::HUMAN.health / 10.0,
        melee_distance: Self::HUMAN.melee_distance,
        melee_distance_angular: Self::HUMAN.melee_distance_angular,
        melee_interval: Self::HUMAN.melee_interval,
        reloading_speed: Self::HUMAN.reloading_speed * 2.0,
        recoil_factor: 6.0,
        bot: BotConfig::ZOMBIE,
        images: &[0, 1, 2],
    };

    pub const ZOMBIE_AGILE: Self = Self {
        kind: ActorKind::Zombie,
        name: "zombie_agile",
        movement_velocity: Self::HUMAN.movement_velocity * 0.8,
        rotation_velocity: 4.0,
        stamina: Duration::from_secs(60),
        health: Self::ZOMBIE.health / 2.0,
        radius: 0.19,
        mass: 45.0,
        melee_damage: Self::ZOMBIE.melee_damage / 2.0,
        bot: BotConfig::ZOMBIE_AGILE,
        images: &[0],
        ..Self::ZOMBIE
    };

    pub fn get_assets_path(&self) -> String {
        return format!("actors/{}", self.name);
    }

    pub fn get_image_path(&self, mut suffix: u8) -> String {
        if !self.images.contains(&suffix) {
            suffix = self.images.first().copied().unwrap_or(0);
        }

        return format!("{}/image_{}.png", self.get_assets_path(), suffix);
    }
}

#[derive(Component)]
pub struct ActorWeaponSprite;
