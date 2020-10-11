pub const PI_2_0: f32 = std::f32::consts::PI * 2.0;

pub fn normalize_radians(radians: f32) -> f32 {
    return radians % PI_2_0;
}

pub fn get_radians_difference(a: f32, b: f32) -> f32 {
    let mut difference = b - a;

    if f32::abs(difference) > std::f32::consts::PI {
        if a < b {
            difference -= PI_2_0;
        } else {
            difference += PI_2_0;
        }
    }

    return normalize_radians(difference);
}

pub fn are_close(x1: f32, y1: f32, x2: f32, y2: f32, distance: f32) -> bool {
    return (x1 - x2).abs() + (y1 - y2).abs() < distance * distance;
}
