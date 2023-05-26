use crate::util::Envelope;
use bevy::ecs::component::Component;
use derive_more::Constructor;
use std::time::Duration;

#[derive(Constructor, Component)]
pub struct Notification {
    pub created: Duration,
}

impl Notification {
    const ENVELOPE: Envelope = Envelope::new(
        Duration::from_millis(150),
        Duration::from_millis(2500),
        Duration::from_millis(500),
    );

    pub fn alpha(&self, time: Duration) -> f32 {
        return Self::ENVELOPE.get(time.saturating_sub(self.created));
    }

    pub fn is_expired(&self, time: Duration) -> bool {
        return time > self.created + Self::ENVELOPE.duration();
    }
}
