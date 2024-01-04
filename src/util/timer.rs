use std::time::Duration;

#[derive(Default)]
pub struct Timer {
    next: Duration,
}

impl Timer {
    pub fn next_if_ready(&mut self, now: Duration, interval: fn() -> Duration) -> bool {
        if self.next < now {
            let is_first = self.next.is_zero();
            self.next = now + interval();
            return !is_first;
        } else {
            return false;
        }
    }

    pub fn set(&mut self, next: Duration) {
        self.next = next;
    }

    pub fn is_ready_or_disabled(&self, now: Duration) -> bool {
        return self.next < now;
    }
}
