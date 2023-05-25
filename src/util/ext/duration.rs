use std::time::Duration;

pub trait DurationExt {
    fn get_progress(self, min: Duration, max: Duration) -> f32;
}

impl DurationExt for Duration {
    fn get_progress(self, min: Duration, max: Duration) -> f32 {
        let elapsed = self.saturating_sub(min).as_secs_f64();
        let range = max.saturating_sub(min).as_secs_f64();

        if elapsed >= range {
            return 1.0;
        } else {
            return (elapsed / range) as f32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_progress(duration: u64, min: u64, max: u64) -> f32 {
        let duration_min = Duration::from_millis(min);
        let duration_max = Duration::from_millis(max);
        return Duration::from_millis(duration).get_progress(duration_min, duration_max);
    }

    #[test]
    fn test_get_progress() {
        assert_eq!(0.0, get_progress(4, 4, 8), "Min");
        assert_eq!(1.0, get_progress(8, 4, 8), "Middle");
        assert_eq!(0.5, get_progress(6, 4, 8), "Max");
        assert_eq!(0.0, get_progress(2, 4, 8), "Smaller than min");
        assert_eq!(1.0, get_progress(10, 4, 8), "Greater than max");
        assert_eq!(1.0, get_progress(5, 5, 5), "All equal");
        assert_eq!(1.0, get_progress(0, 0, 0), "Zeros");
        assert_eq!(1.0, get_progress(5, 0, 0), "Zeros and greater than max");
    }
}
