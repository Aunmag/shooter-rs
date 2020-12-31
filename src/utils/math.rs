use std::f32::consts::PI;
use std::f32::consts::TAU;

pub fn normalize_radians(radians: f32) -> f32 {
    return radians % TAU;
}

pub fn get_radians_difference(a: f32, b: f32) -> f32 {
    let mut difference = b - a;

    if difference.abs() > PI {
        if a < b {
            difference -= TAU;
        } else {
            difference += TAU;
        }
    }

    return normalize_radians(difference);
}

pub fn are_closer_than(x1: f32, y1: f32, x2: f32, y2: f32, distance: f32) -> bool {
    let distance_x = x1 - x2;
    let distance_y = y1 - y2;
    return distance_x * distance_x + distance_y * distance_y < distance * distance;
}

pub fn angle(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    return (y1 - y2).atan2(x1 - x2);
}
