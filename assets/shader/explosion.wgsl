#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct Uniform {
    alpha: f32,
};

@group(2) @binding(0)
var<uniform> uniform: Uniform;

@group(2) @binding(1)
var texture: texture_2d<f32>;

@group(2) @binding(1)
var oputput: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var c = length(in.uv.xy - 0.5) * 2.0;
    var w = pow(c, 64.0);
    var a_inner = pow(c, 2.0);
    var a_outer = saturate((1.0 - c) * 100.0);
    return vec4<f32>(1.0, 1.0, w, a_inner * a_outer * uniform.alpha);
}
