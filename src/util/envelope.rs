use std::time::Duration;

pub struct Envelope {
    pub attack: Duration,
    pub hold: Duration,
    pub release: Duration,
}

impl Envelope {
    pub const fn new(attack: Duration, hold: Duration, release: Duration) -> Self {
        return Self {
            attack,
            hold,
            release,
        };
    }

    pub fn get(&self, elapsed: Duration) -> f32 {
        let mut value = elapsed.as_secs_f32() / self.attack.as_secs_f32();

        if value > 1.0 {
            value = elapsed
                .saturating_sub(self.attack)
                .saturating_sub(self.hold)
                .as_secs_f32();

            value = f32::max(1.0 - value / self.release.as_secs_f32(), 0.0);
        }

        return value;
    }

    pub fn duration(&self) -> Duration {
        return self.attack + self.hold + self.release;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let e = Envelope::new(
            Duration::from_secs(3), // 0 - 3
            Duration::from_secs(3), // 3 - 6
            Duration::from_secs(3), // 6 - 9
        );

        assert_eq!(0.0, e.get(Duration::from_secs_f32(0.0)), "attack start");
        assert_eq!(0.5, e.get(Duration::from_secs_f32(1.5)), "attack mid");
        assert_eq!(
            1.0,
            e.get(Duration::from_secs_f32(3.0)),
            "attack end, hold start"
        );
        assert_eq!(1.0, e.get(Duration::from_secs_f32(4.5)), "hold min");
        assert_eq!(
            1.0,
            e.get(Duration::from_secs_f32(6.0)),
            "hold end, release start"
        );
        assert_eq!(0.5, e.get(Duration::from_secs_f32(7.5)), "release mid");
        assert_eq!(0.0, e.get(Duration::from_secs_f32(9.0)), "release end");
        assert_eq!(0.0, e.get(Duration::from_secs_f32(10.0)), "outside");
    }
}
