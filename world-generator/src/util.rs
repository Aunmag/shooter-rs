pub fn interpolate(a: f32, b: f32, blend: f32) -> f32 {
    return (b - a) * blend + a;
}
