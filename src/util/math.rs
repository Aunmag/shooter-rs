use bevy::prelude::Vec2;
use std::{
    f32::consts::{PI, TAU},
    ops::Neg,
};

pub fn round_by(value: f32, round: f32) -> f32 {
    return (value / round).round() * round;
}

pub fn floor_by(value: f32, floor: f32) -> f32 {
    return (value / floor).floor() * floor;
}

pub fn interpolate(min: f32, max: f32, blend: f32) -> f32 {
    return min + (max - min) * blend.clamp(0.0, 1.0);
}

pub fn interpolate_unbounded(min: f32, max: f32, blend: f32) -> f32 {
    return min + (max - min) * blend;
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

pub fn find_meet_point(
    origin_position: Vec2,
    origin_velocity: f32,
    target_position: Vec2,
    target_velocity: Vec2,
) -> Vec2 {
    if target_velocity.length_squared() == 0.0 {
        return target_position;
    }

    let origin_velocity_opposite = origin_velocity * target_velocity.normalize().neg();
    let distance = (target_position - origin_position).length_squared();
    let velocity = (target_velocity - origin_velocity_opposite).length_squared();
    let advance = (distance / velocity).sqrt();

    return target_velocity * advance + target_position;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_PI_2;

    #[test]
    fn test_normalize_radians() {
        let test = |n: f32| fmt(normalize_radians(n));
        assert_eq!(test(-TAU), fmt(-0.0));
        assert_eq!(test(-PI - FRAC_PI_2), fmt(FRAC_PI_2));
        assert_eq!(test(-PI), fmt(-PI));
        assert_eq!(test(-FRAC_PI_2), fmt(-FRAC_PI_2));
        assert_eq!(test(0.0), fmt(0.0));
        assert_eq!(test(FRAC_PI_2), fmt(FRAC_PI_2));
        assert_eq!(test(PI), fmt(PI));
        assert_eq!(test(PI + FRAC_PI_2), fmt(-FRAC_PI_2));
        assert_eq!(test(TAU), fmt(0.0));
    }

    #[test]
    fn test_angle_difference() {
        let test = |a: f32, b: f32| fmt(angle_difference(a, b));

        // no difference, same values
        assert_eq!(fmt(0.0), test(0.0, 0.0));
        assert_eq!(fmt(0.0), test(1.0, 1.0));
        assert_eq!(fmt(0.0), test(-1.0, -1.0));
        assert_eq!(fmt(0.0), test(7.0, 7.0));
        assert_eq!(fmt(0.0), test(-7.0, -7.0));

        // no difference, different values
        assert_eq!(fmt(0.0), test(0.0, TAU));
        assert_eq!(fmt(-0.0), test(0.0, -TAU));
        assert_eq!(fmt(-0.0), test(TAU, 0.0));
        assert_eq!(fmt(0.0), test(-TAU, 0.0));

        // simple difference
        assert_eq!(fmt(PI), test(0.0, PI));
        assert_eq!(fmt(-PI), test(PI, 0.0));
        assert_eq!(fmt(-PI), test(0.0, -PI));
        assert_eq!(fmt(PI), test(-PI, 0.0));

        // more complex difference
        let third = TAU / 3.0;
        assert_eq!(fmt(-third), test(-third, third));
        assert_eq!(fmt(third), test(third, -third));
        let third_doubled = third * 2.0;
        assert_eq!(fmt(third), test(-third_doubled, third_doubled));
        assert_eq!(fmt(-third), test(third_doubled, -third_doubled));
    }

    fn fmt(n: f32) -> String {
        return format!("{:.6}", n);
    }
}
