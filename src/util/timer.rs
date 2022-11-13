use std::time::Duration;

pub struct Timer {
    duration: Duration,
    target: Duration,
}

impl Timer {
    pub const fn new(duration: Duration) -> Self {
        return Self {
            duration,
            target: Duration::ZERO,
        };
    }

    pub fn next_if_done(&mut self, now: Duration) -> bool {
        if now >= self.target {
            self.target = now + self.duration;
            return true;
        } else {
            return false;
        }
    }
}
