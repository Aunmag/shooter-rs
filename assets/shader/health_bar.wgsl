#import bevy_pbr::utils

const pi = 3.14159;

struct Uniform {
    value: f32,
    color: vec4<f32>,
    thickness: f32,
};

@group(1) @binding(0)
var<uniform> uniform: Uniform;

@group(1) @binding(1)
var texture: texture_2d<f32>;

@group(1) @binding(2)
var oputput: sampler;

fn rotate(v: vec2<f32>, center: vec2<f32>, r: f32) -> vec2<f32> {
    var s = sin(r);
    var c = cos(r);
    return (v - center) * mat2x2(c, -s, s, c) + center;
}

fn ddxy(v: f32, c: f32) -> f32 {
    // TODO: avoid conditions
    // TODO: make it more smooth
    if v < c {
        return 0.0;
    }

    return 1.0;
}

fn ring(value: f32, center: vec2<f32>, color: vec4<f32>) -> vec4<f32> {
    // ring
    var c = length(center);
    c -= 0.5 - uniform.thickness;
    c = abs(c);
    c = ddxy(c, uniform.thickness);
    c = 1.0 - c;

    // trim
    var r = rotate(center, vec2<f32>(0.0, 0.0), -pi * (value + 1.5));
    var t = atan2(r.x, r.y);
    t += pi;
    t = value * pi * 2.0 - t;
    t = ddxy(t, 0.0);
    t = 1.0 - t;

    return vec4<f32>(color.xyz, color.w * clamp(c - t, 0.0, 1.0));
}

@fragment
fn fragment(
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    var center = uv.xy - 0.5;
    var range = 1.0 / 3.0 * 2.0;

    var ring_full = ring(range, center, vec4<f32>(0.0, 0.0, 0.0, 1.0 / 3.0));
    var ring_left = ring(uniform.value * range, center, uniform.color);
    var ring_final = ring_left;

    // TODO: avoid conditions
    if ring_left.w < 0.1 {
        ring_final = ring_full;
    }

    return vec4<f32>(ring_final);
}
