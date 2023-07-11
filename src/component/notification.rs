use crate::util::Envelope;
use bevy::ecs::component::Component;
use std::time::Duration;

#[derive(Component)]
pub struct Notification {
    created: Duration,
    envelope: Envelope,
}

impl Notification {
    const FADE_IN: Duration = Duration::from_millis(150);
    const FADE_OUT: Duration = Duration::from_millis(150);
    const DURATION: Duration = Duration::from_millis(2500);

    pub fn new(created: Duration) -> Self {
        return Self::new_with_duration(created, Self::DURATION);
    }

    pub fn new_with_duration(created: Duration, duration: Duration) -> Self {
        return Self {
            created,
            envelope: Envelope::new(Self::FADE_IN, duration, Self::FADE_OUT),
        };
    }

    pub fn alpha(&self, time: Duration) -> f32 {
        return self.envelope.get(time.saturating_sub(self.created));
    }

    pub fn is_expired(&self, time: Duration) -> bool {
        return time > self.created + self.envelope.duration();
    }
}
