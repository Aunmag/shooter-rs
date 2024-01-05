use std::time::Duration;

#[derive(Default)]
pub struct Timer {
    next: Duration,
}

impl Timer {
    pub fn next_if_ready(&mut self, now: Duration, interval: fn() -> Duration) -> bool {
        if self.next < now {
            let was_enabled = self.is_disabled();
            self.set(now + interval());
            return !was_enabled;
        } else {
            return false;
        }
    }

    pub fn set(&mut self, next: Duration) {
        self.next = next;
    }

    pub fn disable(&mut self) {
        self.set(Duration::ZERO);
    }

    pub fn is_enabled(&self) -> bool {
        return !self.next.is_zero();
    }

    pub fn is_disabled(&self) -> bool {
        return self.next.is_zero();
    }

    pub fn is_ready_or_disabled(&self, now: Duration) -> bool {
        return self.next < now;
    }

    pub fn is_ready_and_enabled(&self, now: Duration) -> bool {
        return !self.is_disabled() && self.next < now;
    }
}
