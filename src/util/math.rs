use bevy::prelude::Vec2;
use std::f32::consts::PI;
use std::f32::consts::TAU;
use std::ops::Neg;

pub fn interpolate(min: f32, max: f32, ratio: f32) -> f32 {
    return min + (max - min) * ratio;
}

pub fn angle_difference(a: f32, b: f32) -> f32 {
    return normalize_radians(b - a);
}

pub fn normalize_radians(mut radians: f32) -> f32 {
    radians %= TAU;

    if radians > PI {
        radians -= TAU;
    } else if radians < -PI {
        radians += TAU;
    }

    return radians;
}

pub fn compress_radians(value: f32) -> u8 {
    if value > 0.0 {
        return (value / TAU * f32::from(u8::MAX)).round() as u8;
    } else {
        return u8::MAX - (-value / TAU * f32::from(u8::MAX)).round() as u8;
    }
}

pub fn decompress_radians(value: u8) -> f32 {
    return value as f32 / f32::from(u8::MAX) * TAU;
}

pub fn find_meet_point(
    origin_position: Vec2,
    origin_velocity: Vec2,
    target_position: Vec2,
    target_velocity: Vec2,
) -> Vec2 {
    if target_velocity.length_squared() == 0.0 {
        return target_position;
    }

    let origin_velocity_opposite = origin_velocity.length() * target_velocity.normalize().neg();
    let distance = (target_position - origin_position).length_squared();
    let velocity = (target_velocity - origin_velocity_opposite).length_squared();
    let advance = (distance / velocity).sqrt();

    return target_velocity * advance + target_position;
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use std::f32::consts::FRAC_PI_2;

    #[test]
    fn test_normalize_radians() {
        assert_abs_diff_eq!(normalize_radians(-TAU), 0.0);
        assert_abs_diff_eq!(normalize_radians(-PI - FRAC_PI_2), FRAC_PI_2);
        assert_abs_diff_eq!(normalize_radians(-PI), -PI);
        assert_abs_diff_eq!(normalize_radians(-FRAC_PI_2), -FRAC_PI_2);
        assert_abs_diff_eq!(normalize_radians(0.0), 0.0);
        assert_abs_diff_eq!(normalize_radians(FRAC_PI_2), FRAC_PI_2);
        assert_abs_diff_eq!(normalize_radians(PI), PI);
        assert_abs_diff_eq!(normalize_radians(PI + FRAC_PI_2), -FRAC_PI_2);
        assert_abs_diff_eq!(normalize_radians(TAU), 0.0);
    }

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
