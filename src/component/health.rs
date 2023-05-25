use bevy::ecs::component::Component;
use std::time::Duration;

const DECAY_TIME: Duration = Duration::from_millis(500);

#[derive(Component)]
pub struct Health {
    value: f32,
    value_previous: f32,
    resistance: f32,
    death_time: Duration,
}

impl Health {
    pub fn new(mut resistance: f32) -> Self {
        if resistance < 1.0 {
            log::warn!(
                "Resistance should be >= 1.0 but got {} and thus set to 1.0",
                resistance
            );

            resistance = 1.0;
        }

        return Self {
            value: 1.0,
            value_previous: 1.0,
            resistance,
            death_time: Duration::from_secs(0),
        };
    }

    pub fn save_change(&mut self) {
        self.value_previous = self.value;
    }

    pub fn damage(&mut self, value: f32, now: Duration) {
        self.set(self.value - value / self.resistance, now);
    }

    pub fn heal(&mut self) {
        if self.is_alive() {
            self.value = 1.0;
        }
    }

    pub fn set(&mut self, value: f32, now: Duration) {
        let was_alive = self.is_alive();
        self.value = value.clamp(0.0, 1.0);

        if !self.is_alive() && was_alive {
            self.death_time = now;
        }
    }

    pub fn get(&self) -> f32 {
        return self.value;
    }

    pub fn change(&self) -> f32 {
        return self.value - self.value_previous;
    }

    pub fn is_alive(&self) -> bool {
        return self.value > 0.0;
    }

    pub fn is_decayed(&self, now: Duration) -> bool {
        if self.is_alive() {
            return false;
        } else {
            return now > self.death_time + DECAY_TIME;
        }
    }
}
