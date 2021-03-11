use std::time::Duration;

pub trait DurationExt {
    fn sub_safely(self, other: Duration) -> Duration;

    fn get_progress(self, min: Duration, max: Duration) -> f32;
}

impl DurationExt for Duration {
    // TODO: Use `Duration::saturating_sub` in future
    fn sub_safely(self, other: Duration) -> Duration {
        if self > other {
            return self - other;
        } else {
            return Duration::from_millis(0);
        }
    }

    fn get_progress(self, min: Duration, max: Duration) -> f32 {
        let elapsed = self.sub_safely(min).as_secs_f64();
        let range = max.sub_safely(min).as_secs_f64();

        if elapsed >= range {
            return 1.0;
        } else {
            #[allow(clippy::cast_possible_truncation)]
            {
                return (elapsed / range) as f32;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_progress() {
        assert_eq!(
            0.0,
            Duration::from_millis(4).get_progress(
                Duration::from_millis(4),
                Duration::from_millis(8),
            ),
            "Min",
        );

        assert_eq!(
            1.0,
            Duration::from_millis(8).get_progress(
                Duration::from_millis(4),
                Duration::from_millis(8),
            ),
            "Middle",
        );

        assert_eq!(
            0.5,
            Duration::from_millis(6).get_progress(
                Duration::from_millis(4),
                Duration::from_millis(8),
            ),
            "Max",
        );

        assert_eq!(
            0.0,
            Duration::from_millis(2).get_progress(
                Duration::from_millis(4),
                Duration::from_millis(8),
            ),
            "Smaller than min",
        );

        assert_eq!(
            1.0,
            Duration::from_millis(10).get_progress(
                Duration::from_millis(4),
                Duration::from_millis(8),
            ),
            "Greater than max",
        );

        assert_eq!(
            1.0,
            Duration::from_millis(5).get_progress(
                Duration::from_millis(5),
                Duration::from_millis(5),
            ),
            "All equal",
        );

        assert_eq!(
            1.0,
            Duration::from_millis(0).get_progress(
                Duration::from_millis(0),
                Duration::from_millis(0),
            ),
            "Zeros",
        );

        assert_eq!(
            1.0,
            Duration::from_millis(5).get_progress(
                Duration::from_millis(0),
                Duration::from_millis(0),
            ),
            "Zeros and greater than max",
        );
    }
}
