#import bevy_render::maths::PI
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const RANGE = 0.75;
const COLOR_VOID = vec4<f32>(0.0, 0.0, 0.0, 0.333333);
const COLOR_STAMINA = vec4<f32>(0.8, 0.8, 0.8, 0.4);
const COLOR_HEALTH = vec4<f32>(1.0, 0.0, 0.0, 0.6);
const COLOR_AMMO = vec4<f32>(0.8, 0.8, 0.8, 0.4);

struct Material {
    health: f32,
    health_alpha: f32,
    ammo: f32,
    ammo_alpha: f32,
    stamina: f32,
};

@group(2) @binding(0)
var<uniform> material: Material;

fn rotate(v: vec2<f32>, center: vec2<f32>, r: f32) -> vec2<f32> {
    let s = sin(r);
    let c = cos(r);
    return (v - center) * mat2x2(c, -s, s, c) + center;
}

fn mix_alpha(color: vec4<f32>, a: f32) -> vec4<f32> {
    return vec4<f32>(color.xyz, color.w * a);
}

fn ring(center: vec2<f32>, value: f32, radius: f32, thickness: f32) -> f32 {
    // ring
    var c = length(center);
    c -= (radius / 2.0 - thickness);
    c = abs(c);
    c = step(thickness, c);
    c = 1.0 - c;

    // trim
    let r = rotate(center, vec2<f32>(0.0, 0.0), -PI * (value + 1.5));
    var t = atan2(r.x, r.y);
    t += PI;
    t = value * PI * 2.0 - t;
    t = step(0.0, t);
    t = 1.0 - t;

    return clamp(c - t, 0.0, 1.0);
}

fn bar(value: f32, center: vec2<f32>, color: vec4<f32>, radius: f32, thickness: f32) -> vec4<f32> {
    let fill = ring(center, value * RANGE, radius, thickness);

    if fill > 0.1 {
        return mix_alpha(color, fill);
    } else {
        return mix_alpha(COLOR_VOID, ring(center, RANGE, radius, thickness));
    }
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let center  = in.uv.xy - 0.5;
    let stamina = bar(material.stamina, center, mix_alpha(COLOR_STAMINA, 1.0                  ), 0.62, 0.01);
    let health  = bar(material.health , center, mix_alpha(COLOR_HEALTH , material.health_alpha), 0.91, 0.06);
    let ammo    = bar(material.ammo   , center, mix_alpha(COLOR_AMMO   , material.ammo_alpha  ), 1.00, 0.01);
    return vec4<f32>(stamina + health + ammo);
}
