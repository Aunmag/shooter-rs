use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Health {
    value: f32,
    value_previous: f32,
    resistance: f32,
}

impl Health {
    const LOW_VALUE: f32 = 0.4;

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
        };
    }

    pub fn damage(&mut self, damage: f32) {
        self.value = (self.value - damage / self.resistance).clamp(0.0, 1.0);
    }

    pub fn heal(&mut self) {
        if self.is_alive() {
            self.value = 1.0;
        }
    }

    /// NOTE: only heath system can call this method to commit health changes
    pub fn commit(&mut self) {
        self.value_previous = self.value;
    }

    pub fn get(&self) -> f32 {
        return self.value;
    }

    pub fn get_damage(&self) -> f32 {
        return self.value_previous - self.value;
    }

    pub fn is_alive(&self) -> bool {
        return self.value > 0.0;
    }

    pub fn is_just_died(&self) -> bool {
        return !self.is_alive() && self.get_damage() > 0.0;
    }

    pub fn is_low(&self) -> bool {
        return self.value < Self::LOW_VALUE;
    }
}
