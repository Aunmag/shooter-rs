use crate::{
    plugin::actor::{action::ActorActions, config::ActorConfig, ActorActionsExt},
    util::ext::{DurationExt, Vec2Ext},
};
use bevy::{ecs::component::Component, math::Vec2};
use std::time::Duration;

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

impl Actor {
    pub const ARMS_LENGTH_1: f32 = 0.546875;
    pub const ARMS_LENGTH_2: f32 = 0.34375;

    pub const fn new(config: &'static ActorConfig, skill: f32) -> Self {
        return Self {
            config,
            skill,
            stamina: 1.0,
            movement: Vec2::ZERO,
            actions: ActorActions::empty(),
            look_at: None,
            melee_next: Duration::ZERO,
        };
    }

    pub fn reset_actions(&mut self) {
        self.movement = Vec2::ZERO;
        self.actions = ActorActions::empty();
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

#[derive(Component)]
pub struct ActorWeaponSprite;
