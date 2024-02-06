use bevy::ecs::component::Component;
use std::time::Duration;

const STUN_DURATION: Duration = Duration::from_millis(700);

/// NOTE: health must not be affected by skill, excepting player
#[derive(Component)]
pub struct Health {
    value_max: f32,
    value: f32,
    value_previous: f32,
    stun_timer: Duration,
}

impl Health {
    const LOW_VALUE_NORMALIZED: f32 = 0.4;

    pub fn new(value_max: f32) -> Self {
        return Self {
            value_max,
            value: value_max,
            value_previous: value_max,
            stun_timer: Duration::ZERO,
        };
    }

    pub fn multiply(&mut self, n: f32) {
        self.value_max *= n;
        self.value *= n;
        self.value_previous *= n;
    }

    pub fn damage(&mut self, damage: f32, time: Duration) {
        self.value = (self.value - damage).clamp(0.0, self.value_max);

        let stun_timer = time + STUN_DURATION.mul_f32(self.get_damage_normalized());

        if self.stun_timer < stun_timer {
            self.stun_timer = stun_timer;
        }
    }

    pub fn heal(&mut self) {
        if self.is_alive() {
            self.value = self.value_max;
        }
    }

    /// NOTE: only heath system can call this method to commit health changes
    pub fn commit(&mut self) {
        self.value_previous = self.value;
    }

    pub fn get_normalized(&self) -> f32 {
        return self.value / self.value_max;
    }

    pub fn get_damage(&self) -> f32 {
        return self.value_previous - self.value;
    }

    pub fn get_damage_normalized(&self) -> f32 {
        return self.get_damage() / self.value_max;
    }

    pub fn is_alive(&self) -> bool {
        return self.value > 0.0;
    }

    pub fn is_just_died(&self) -> bool {
        return !self.is_alive() && self.get_damage() > 0.0;
    }

    pub fn is_low(&self) -> bool {
        return self.get_normalized() < Self::LOW_VALUE_NORMALIZED;
    }

    pub fn is_stunned(&self, time: Duration) -> bool {
        return time < self.stun_timer;
    }
}
