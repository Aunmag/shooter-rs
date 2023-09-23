use std::time::Duration;

pub trait DurationExt {
    fn delta(&self, delta: f32) -> f32;

    fn progress(self, min: Duration, max: Duration) -> f32;
}

impl DurationExt for Duration {
    fn delta(&self, delta: f32) -> f32 {
        return f32::min(delta / self.as_secs_f32(), 1.0);
    }

    fn progress(self, min: Duration, max: Duration) -> f32 {
        let elapsed = self.saturating_sub(min);
        let range = max.saturating_sub(min);

        if elapsed >= range || range.is_zero() {
            return 1.0;
        } else {
            return (elapsed.as_secs_f64() / range.as_secs_f64()) as f32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn progress(duration: u64, min: u64, max: u64) -> f32 {
        let duration_min = Duration::from_millis(min);
        let duration_max = Duration::from_millis(max);
        return Duration::from_millis(duration).progress(duration_min, duration_max);
    }

    #[test]
    fn test_get_progress() {
        assert_eq!(0.0, progress(4, 4, 8), "Min");
        assert_eq!(1.0, progress(8, 4, 8), "Middle");
        assert_eq!(0.5, progress(6, 4, 8), "Max");
        assert_eq!(0.0, progress(2, 4, 8), "Smaller than min");
        assert_eq!(1.0, progress(10, 4, 8), "Greater than max");
        assert_eq!(1.0, progress(5, 5, 5), "All equal");
        assert_eq!(1.0, progress(0, 0, 0), "Zeros");
        assert_eq!(1.0, progress(5, 0, 0), "Zeros and greater than max");
    }
}
