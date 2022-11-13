use std::f32::consts::PI;
use std::f32::consts::TAU;

pub fn angle_difference(a: f32, b: f32) -> f32 {
    let difference = (b - a) % TAU;

    if difference > PI {
        return difference - TAU;
    } else if difference < -PI {
        return difference + TAU;
    } else {
        return difference;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_angle_difference() {
        // no difference, same values
        assert_abs_diff_eq!(0.0, angle_difference(0.0, 0.0));
        assert_abs_diff_eq!(0.0, angle_difference(1.0, 1.0));
        assert_abs_diff_eq!(0.0, angle_difference(-1.0, -1.0));
        assert_abs_diff_eq!(0.0, angle_difference(7.0, 7.0));
        assert_abs_diff_eq!(0.0, angle_difference(-7.0, -7.0));

        // no difference, different values
        assert_abs_diff_eq!(0.0, angle_difference(0.0, TAU));
        assert_abs_diff_eq!(0.0, angle_difference(0.0, -TAU));
        assert_abs_diff_eq!(0.0, angle_difference(TAU, 0.0));
        assert_abs_diff_eq!(0.0, angle_difference(-TAU, 0.0));

        // simple difference
        assert_abs_diff_eq!(PI, angle_difference(0.0, PI));
        assert_abs_diff_eq!(-PI, angle_difference(PI, 0.0));
        assert_abs_diff_eq!(-PI, angle_difference(0.0, -PI));
        assert_abs_diff_eq!(PI, angle_difference(-PI, 0.0));

        // more complex difference
        let third = TAU / 3.0;
        assert_abs_diff_eq!(-third, angle_difference(-third, third));
        assert_abs_diff_eq!(third, angle_difference(third, -third));
        let third_doubled = third * 2.0;
        assert_abs_diff_eq!(third, angle_difference(-third_doubled, third_doubled));
        assert_abs_diff_eq!(-third, angle_difference(third_doubled, -third_doubled));
    }
}
