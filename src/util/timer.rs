use std::time::Duration;

pub struct Timer {
    next: Duration,
}

impl Default for Timer {
    fn default() -> Self {
        return Self {
            next: Duration::MAX,
        };
    }
}

impl Timer {
    pub fn try_next_add(&mut self, now: Duration, interval: Duration) -> bool {
        if self.is_ready(now) {
            self.next += interval;
            return true;
        } else if self.is_disabled() {
            self.set(now + interval);
            return false;
        } else {
            return false;
        }
    }

    pub fn try_next_set(&mut self, now: Duration, interval: fn() -> Duration) -> bool {
        if self.is_ready(now) {
            self.set(now + interval());
            return true;
        } else if self.is_disabled() {
            self.set(now + interval());
            return false;
        } else {
            return false;
        }
    }

    pub fn set(&mut self, next: Duration) {
        self.next = next;
    }

    pub fn disable(&mut self) {
        self.next = Duration::MAX;
    }

    pub fn is_ready(&self, now: Duration) -> bool {
        return self.next < now;
    }

    pub fn is_enabled(&self) -> bool {
        return !self.is_disabled();
    }

    pub fn is_disabled(&self) -> bool {
        return self.next == Duration::MAX;
    }
}
