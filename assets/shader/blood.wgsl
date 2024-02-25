#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const ALPHA = 0.8;
const SCALE = 8.0;
const NOISE_ROUGHNESS_1 = 0.5;
const NOISE_ROUGHNESS_2 = 2.5;
const CONTRAST = 3.9;
const CENTER_THICKNESS_1 = 1.2;
const CENTER_THICKNESS_2 = 4.0;
const CENTER_BRIGHTNESS_1 = 2.0;
const CENTER_BRIGHTNESS_2 = 1.4;
const KF = 0.2;

struct Uniform {
    seed: f32,
    size: f32,
};

@group(1) @binding(0)
var<uniform> uniform: Uniform;

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var oputput: sampler;

fn g(p: vec2<f32>) -> vec2<f32> {
    return sin(p.x * p.y + vec2<f32>(0.0, 1.571));
}

fn noise(p: vec2<f32>) -> f32 {
    var i = floor(p);
    var f = fract(p);
    f = f * f * (3.0 - 2.0 * f);

    return mix(
        mix(
            sin(KF * dot(p, g(i + vec2(0.0, 0.0)))),
            sin(KF * dot(p, g(i + vec2(1.0, 0.0)))),
            f.x
        ),
        mix(
            sin(KF * dot(p, g(i + vec2(0.0, 1.0)))),
            sin(KF * dot(p, g(i + vec2(1.0, 1.0)))),
            f.x
        ),
        f.y
    );
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv_m = in.uv.xy;
    uv_m *= uniform.size / SCALE;
    uv_m += uniform.seed;

    var m = mat2x2(1.6, 1.2, -1.2, 1.6);
    var f = 0.0;
    f += NOISE_ROUGHNESS_1 * noise(uv_m); uv_m = m * uv_m;
    f += NOISE_ROUGHNESS_1 * NOISE_ROUGHNESS_2 / 2.0 * noise(uv_m); uv_m = m * uv_m;
    f += NOISE_ROUGHNESS_1 * NOISE_ROUGHNESS_2 / 4.0 * noise(uv_m); uv_m = m * uv_m;
    f += NOISE_ROUGHNESS_1 * NOISE_ROUGHNESS_2 / 8.0 * noise(uv_m); uv_m = m * uv_m;
    f = 0.5 + 0.5 * f;

    var c = length(vec2(0.5, 0.5) - in.uv.xy) * 2.0;
    f = pow(f, pow(c, CENTER_THICKNESS_1) * CENTER_THICKNESS_2);
    f *= (1.0 - pow(c, CENTER_BRIGHTNESS_1)) * CENTER_BRIGHTNESS_2;
    f = pow(f, CONTRAST);
    f = clamp(f, 0.0, 1.0);

    return vec4<f32>(pow(vec3<f32>(0.52, 0.09, 0.09), vec3(2.2)), f * ALPHA);
}
