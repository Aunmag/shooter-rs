#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct Material {
    alpha: f32,
};

@group(2) @binding(0)
var<uniform> material: Material;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let c = length(in.uv.xy - 0.5) * 2.0;
    let w = pow(c, 64.0);
    let a_inner = pow(c, 2.0);
    let a_outer = saturate((1.0 - c) * 100.0);
    return vec4<f32>(1.0, 1.0, w, a_inner * a_outer * material.alpha);
}
