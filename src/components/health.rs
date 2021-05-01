use crate::utils;
use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;
use std::time::Duration;

const DECAY_TIME: Duration = Duration::from_millis(500);

pub struct Health {
    value: f32,
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

        return Health {
            value: 1.0,
            resistance,
            death_time: Duration::from_secs(0),
        };
    }

    pub fn damage(&mut self, value: f32, now: Duration) {
        self.set(self.value - value / self.resistance, now);
    }

    pub fn set(&mut self, value: f32, now: Duration) {
        let was_alive = self.is_alive();

        self.value = utils::math::clamp(value, 0.0, 1.0);

        if !self.is_alive() && was_alive {
            self.death_time = now;
        }
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

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}
