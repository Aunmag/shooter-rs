#import bevy_pbr::utils

const RANGE = 0.75;
const COLOR_VOID = vec4<f32>(0.0, 0.0, 0.0, 0.333333);
const COLOR_STAMINA = vec4<f32>(0.8, 0.8, 0.8, 0.4);
const COLOR_HEALTH = vec4<f32>(1.0, 0.0, 0.0, 0.6);
const COLOR_AMMO = vec4<f32>(0.8, 0.8, 0.8, 0.4);

struct Uniform {
    health: f32,
    health_alpha: f32,
    ammo: f32,
    ammo_alpha: f32,
    stamina: f32,
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

fn mix_alpha(color: vec4<f32>, a: f32) -> vec4<f32> {
    return vec4<f32>(color.xyz, color.w * a);
}

fn ring(center: vec2<f32>, value: f32, radius: f32, thickness: f32) -> f32 {
    // ring
    var c = length(center);
    c -= (radius / 2.0 - thickness);
    c = abs(c);
    c = ddxy(c, thickness);
    c = 1.0 - c;

    // trim
    var r = rotate(center, vec2<f32>(0.0, 0.0), -PI * (value + 1.5));
    var t = atan2(r.x, r.y);
    t += PI;
    t = value * PI * 2.0 - t;
    t = ddxy(t, 0.0);
    t = 1.0 - t;

    return clamp(c - t, 0.0, 1.0);
}

fn bar(value: f32, center: vec2<f32>, color: vec4<f32>, radius: f32, thickness: f32) -> vec4<f32> {
    var fill = ring(center, value * RANGE, radius, thickness);

    // TODO: avoid conditions
    if fill > 0.1 {
        return mix_alpha(color, fill);
    } else {
        return mix_alpha(COLOR_VOID, ring(center, RANGE, radius, thickness));
    }
}

@fragment
fn fragment(
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    var center  = uv.xy - 0.5;
    var stamina = bar(uniform.stamina, center, mix_alpha(COLOR_STAMINA, 1.0                 ), 0.62, 0.01);
    var health  = bar(uniform.health , center, mix_alpha(COLOR_HEALTH , uniform.health_alpha), 0.91, 0.06);
    var ammo    = bar(uniform.ammo   , center, mix_alpha(COLOR_AMMO   , uniform.ammo_alpha  ), 1.00, 0.01);
    return vec4<f32>(stamina + health + ammo);
}
